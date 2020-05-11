//! エンコーダ

use crate::range_coder_struct::RangeCoder;
use crate::uext::UEXT;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::Write;
use std::path::Path;

pub struct Encoder {
    range_coder: RangeCoder,
    /// 出力する符号
    data: VecDeque<u8>,
    /// 未確定桁を格納するバッファ
    buffer: Option<u8>,
    /// 0xff or 0x00 になる値の個数
    /// (参考文献でcarryNと呼ばれるもの)
    carry_n: u32,
    /// 終了処理が行われたかを示す
    is_finished: bool,
}
impl Encoder {
    /// 1シンボル、エンコードを進める
    pub fn encode(&mut self, simbol_index: usize) {
        //println!("encode index: {}", simbol_index);

        // 下限、レンジの更新
        let range_new = self
            .range_coder
            .range_when_encode(self.range_coder.simbol_data().simbol_param(simbol_index));
        let lower_bound_new = self
            .range_coder
            .lower_bound_when_encode(self.range_coder.simbol_data().simbol_param(simbol_index));
        self.range_coder.set_range(range_new);
        match lower_bound_new {
            (v, is_overflow) => {
                if is_overflow {
                    /*
                    lower_boundがオーバーフローした場合
                    出力待ち桁はbuffer+1 00 00..00 00に確定する
                    */
                    self.move_buffer_to_data_overflow();
                }
                self.range_coder.set_lower_bound(v);
            }
        }

        //println!("lower_bound is :0x{:x}", self.range_coder.lower_bound());
        //println!("range is       :0x{:x}", self.range_coder.range());
        /*
        上位8bitの判定

        上位8bitが決定した場合、シフトを行い、決定した桁を取り出しておく
        */
        static TOP: u32 = 1 << 24;
        while self.range_coder.range() < TOP {
            // 確定した上位8bit(下限の上位8bit)をbuffer_newに格納
            let buffer_new = (self.range_coder.lower_bound() >> 24) as u8;
            match buffer_new {
                /*
                バッファが1111 1111だった場合
                次の値がとる範囲[L,L+R)で、L+Rの下位24bitの和がoverflowを起こした時に
                ここより上の桁に繰り上がりの影響がでるため、carry_nに含める
                */
                0xff => {
                    self.carry_n += 1;
                }
                // バッファが普通の値だった場合
                _ => {
                    /*
                    繰り上がりにより出力待ちの"buffer ff ff .. ff"が
                    "buffer+1 00 00..00" になる可能性はないから
                    "buffer ff ff..ff" を出力
                    */
                    self.move_buffer_to_data_left_shift(buffer_new);
                    // 次の未決定桁はbuffer_newになる
                }
            }
            // 先頭8bitはバッファに入れたのでシフトして演算精度をあげる
            self.range_coder
                .set_lower_bound(self.range_coder.lower_bound() << 8);
            self.range_coder.set_range(self.range_coder.range() << 8);
        }
        //一文字エンコード完了
    }
    /// 下限更新時のオーバーフローによるバッファの確定
    fn move_buffer_to_data_overflow(&mut self) {
        self.data.push_back(self.buffer.unwrap() + 1);
        for _ in 0..self.carry_n {
            self.data.push_back(0x00);
        }
        self.carry_n = 0;
        self.buffer = None;
    }
    /// 左シフト(範囲拡大)によるバッファの確定
    fn move_buffer_to_data_left_shift(&mut self, new_buffer: u8) {
        if let Some(buff) = self.buffer {
            self.data.push_back(buff);
            for _ in 0..self.carry_n {
                self.data.push_back(0xff);
            }
        }
        self.carry_n = 0;
        self.buffer = Some(new_buffer);
    }
    /// エンコード終了後に呼び出して、
    /// buffer,carry_nを出力する。
    pub fn finish(&mut self) {
        // 未確定のバッファがあれば出力に追加
        match self.buffer {
            Some(b) => {
                self.data.push_back(b);
            }
            None => {}
        }
        // carry_nがあれば0xffで出力に追加
        for _ in 0..self.carry_n {
            self.data.push_back(0xff);
        }
        // 現状の下限を出力
        for _ in 0..4 {
            self.data
                .push_back((self.range_coder.lower_bound() >> 24) as u8);
            self.range_coder
                .set_lower_bound(self.range_coder.lower_bound() << 8);
        }
        self.is_finished = true;
    }
}
impl Encoder {
    pub fn new(range_coder: RangeCoder) -> Self {
        Self {
            data: VecDeque::new(),
            range_coder: range_coder,
            buffer: None,
            carry_n: 0,
            is_finished: false,
        }
    }
    pub fn write(&self, path: &Path) -> Result<(), String> {
        if !self.is_finished {
            println!("終了処理してから書き出してください");
            // mutableな借用でないため自動で解決するようにはできない
        }
        /*print!("\n data is : 0x");
        for v in &self.data {
            print!("{:x}", v);
        }
        */
        println!();
        // ファイルオープン
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => return Result::Err("Error happend creating (or opening) file".to_string()),
        };
        // バッファ宣言
        let mut buff = Vec::new();
        // usizeのサイズ書き込み
        buff.append(&mut vec![std::mem::size_of::<usize>() as u8]);
        // インデックス、出現数を交互に書き込み
        // 出現数が1以上のシンボルの出現数、インデックスをvectorに集める
        let v: Vec<_> = self
            .range_coder
            .simbol_data()
            .simbol_paramaters()
            .iter()
            .enumerate()
            .map(|(i, parm)| (i, parm.c))
            .filter(|(_, c)| *c > 1)
            .collect();
        // 保存するシンボルの数を書き込む
        buff.append(&mut v.len().to_vec_u8());
        // シンボルを書き込む
        for (index, simbol_c) in v {
            buff.append(&mut index.to_vec_u8());
            buff.append(&mut simbol_c.to_vec_u8());
        }
        // 出力データ書き込み
        for &i in &self.data {
            buff.push(i);
        }
        // ファイルに書き込み
        match file.write_all(&buff) {
            Ok(_) => return Result::Ok(()),
            Err(_) => return Result::Err("Some error happened while writing buffer".to_string()),
        };
    }
}
