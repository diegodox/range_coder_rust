//! エンコーダ

use crate::range_coder_struct::RangeCoder;
use crate::simbol_data::SimbolParam;
use std::collections::VecDeque;

pub struct Encoder {
    /// レンジコーダ
    range_coder: RangeCoder,
    /// 出力する符号
    data: VecDeque<u8>,
}
impl Encoder {
    /// 1シンボル、エンコードを進める
    /// 返値は出力したバイト数
    pub fn encode(&mut self, simbol_param: &SimbolParam, total_freq: u32) -> u32 {
        let mut put_byte = 0;
        // 下限、レンジの更新
        self.range_coder.param_update(simbol_param, total_freq);
        /*
        上位8bitの判定

        上位8bitが決定した場合、シフトを行い、決定した桁を取り出しておく
        */
        const TOP8: u64 = 1 << (64 - 8);
        while (self.range_coder.lower_bound())
            ^ (self.range_coder.range() + self.range_coder.lower_bound())
            < TOP8
        {
            println!("桁確定");
            self.data.push_back(self.range_coder.left_shift());
            put_byte += 1
        }
        // レンジが小さくなったら
        // 次の上位8bitが変動しないギリギリまで範囲を絞り出力する
        const TOP16: u64 = 1 << (64 - 16);
        while self.range_coder.range() < TOP16 {
            println!("範囲を絞る");
            let range_new = !self.range_coder.lower_bound() & (TOP16 - 1);
            self.range_coder.set_range(range_new);
            self.data.push_back(self.range_coder.left_shift());
            put_byte += 1;
        }
        println!();
        println!("lobo:0x{:x}", self.range_coder.lower_bound());
        println!("rage:0x{:x}", self.range_coder.range());
        println!();
        //一文字エンコード完了
        put_byte
    }
    /// エンコード終了後に呼び出して、
    /// 下限を出力
    pub fn finish(&mut self) {
        // 現状の下限を出力
        for _ in 0..8 {
            self.data.push_back(self.range_coder.left_shift());
        }
    }
}
impl Encoder {
    pub fn new(range_coder: RangeCoder) -> Self {
        Self {
            data: VecDeque::new(),
            range_coder: range_coder,
        }
    }
    pub fn data(&self) -> &VecDeque<u8> {
        &self.data
    }
    pub(crate) fn range_coder(&self) -> &RangeCoder {
        &self.range_coder
    }
    pub(crate) fn range_coder_mut(&mut self) -> &mut RangeCoder {
        &mut self.range_coder
    }
}
/*
pub fn write(&self, path: &Path) -> Result<(), String> {
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
*/
