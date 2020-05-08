use super::RangeCoder;
use crate::simbol_trait::ForRangeCoder;
use crate::uext::UEXT;
use std::fs::File;
use std::io::prelude::Write;
use std::path::Path;

impl<T> RangeCoder<T>
where
    T: Eq + std::hash::Hash + ForRangeCoder + Ord + std::fmt::Debug + Clone,
{
    /// 1シンボル、エンコードを進める
    pub fn encode(&mut self, simbol: T) {
        // simbolのindexをとる
        let simbol_data = self.simbol_data.get(&simbol).unwrap();
        // Range/totalの一時保存
        let range_before = self.range / self.simbol_data.total as u32;

        // Rangeの更新
        match (simbol_data.cum + simbol_data.c).cmp(&self.simbol_data.total) {
            // レンジ最後のシンボルの場合、通常のレンジ更新で発生する誤差(整数除算によるもの)を含める
            std::cmp::Ordering::Equal => {
                self.range = self.range - (range_before * simbol_data.cum);
            }
            // レンジ最後のシンボルでない場合、通常のレンジ更新を行う
            std::cmp::Ordering::Less => {
                self.range = range_before * simbol_data.c;
            }
            // Graterになることはない
            _ => unreachable!("panic! (cum+c) should not be bigger than total"),
        }
        // lower_boundの更新
        match self
            .lower_bound
            .overflowing_add(range_before * simbol_data.cum)
        {
            (v, true) => {
                /*
                lower_boundがオーバーフローした場合
                出力待ち桁はbuffer+1 00 00..00 00に確定する
                */
                self.move_determined_buffer_to_data(true, None);
                self.lower_bound = v;
            }
            (v, false) => {
                self.lower_bound = v;
            }
        }
        /*
        上位8bitの判定

        上位8bitが決定した場合、シフトを行い、決定した桁を取り出しておく
        */
        static TOP: u32 = 1 << 24;
        while self.range < TOP {
            // 確定した上位8bit(下限の上位8bit)をbuffer_newに格納
            let buffer_new = (self.lower_bound >> 24) as u8;
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
                    self.move_determined_buffer_to_data(false, Some(buffer_new));
                    // 次の未決定桁はbuffer_newになる
                }
            }
            // 先頭8bitはバッファに入れたのでシフトして演算精度をあげる
            self.lower_bound <<= 8;
            self.range <<= 8;
        }
        //一文字エンコード完了
    }
    /// 確定した桁の出力
    fn move_determined_buffer_to_data(&mut self, is_overflow: bool, new_buffer: Option<u8>) {
        match is_overflow {
            true => match self.buffer {
                Some(b) => {
                    self.data.push_back(b + 1);
                    for _ in 0..self.carry_n {
                        self.data.push_back(0x00);
                    }
                }
                None => panic!("未確定の桁がない状態でoverflowしました。"),
            },
            false => match self.buffer {
                Some(b) => {
                    self.data.push_back(b);
                    for _ in 0..self.carry_n {
                        self.data.push_back(0xff);
                    }
                }
                None => {
                    for _ in 0..self.carry_n {
                        self.data.push_back(0xff);
                    }
                }
            },
        }
        self.carry_n = 0;
        self.buffer = new_buffer;
    }
    /// エンコード終了後に呼び出して、
    /// buffer,carry_nを出力する。
    pub fn finish(&mut self) {
        match self.buffer {
            Some(b) => {
                self.data.push_back(b);
            }
            None => {}
        }
        for _ in 0..self.carry_n {
            self.data.push_back(0xff);
        }
        self.data.push_back((self.lower_bound >> 24) as u8);
        self.lower_bound <<= 8;
        self.data.push_back((self.lower_bound >> 24) as u8);
        self.lower_bound <<= 8;
        self.data.push_back((self.lower_bound >> 24) as u8);
        self.lower_bound <<= 8;
        self.data.push_back((self.lower_bound >> 24) as u8);
    }
    /// エンコードデータを書き込み
    ///
    /// データ構造
    /// 名前|先頭バイト|形式
    /// -|-|-
    /// シンボルの種類数|0|u8
    /// シンボルデータ|1|シンボルそのもの(サイズは外部指定)、シンボルの出現数(u32)
    /// 符号|$(size[byte]+4)\times+1$|符号
    pub fn write(&self, path: &Path) -> Result<(), String> {
        // ファイルオープン
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => return Result::Err("Error happend creating (or opening) file".to_string()),
        };
        // バッファ宣言
        let mut buff = Vec::new();

        // シンボルの種類数書き込み
        buff.append(&mut self.simbol_data.simbol_type_count.to_be_bytes().to_vec());
        // シンボル、出現数を交互に書き込み
        let v: Vec<_> = self
            .simbol_data
            .index
            .iter()
            .map(|(k, v)| (k, &self.simbol_data.simbol_paramaters[*v as usize]))
            .collect();
        for (simbol, simbol_param) in v {
            buff.append(&mut simbol.save()); //sizeは可変
            buff.append(&mut simbol_param.c.to_vec_u8()); //sizeは4
        }
        // 出力データ書き込み
        for &i in &self.data {
            buff.push(i);
        }
        // バッファ書き込み
        match file.write_all(&buff) {
            Ok(_) => return Result::Ok(()),
            Err(_) => return Result::Err("Some error happened while writing buffer".to_string()),
        };
    }
}
