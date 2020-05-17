//! エンコーダ

use crate::alphabet_param::AlphabetParam;
use crate::range_coder::RangeCoder;
use std::collections::VecDeque;

pub struct Encoder {
    /// レンジコーダ
    range_coder: RangeCoder,
    /// 出力する符号
    data: VecDeque<u8>,
}
impl Encoder {
    /// 1アルファベット、エンコードを進める
    /// 返値は出力したバイト数
    pub fn encode(&mut self, alphabet_param: &AlphabetParam, total_freq: u32) -> u32 {
        // 出力したバイト数を入れる
        let mut put_byte = 0;
        // 下限、レンジの更新
        self.range_coder.param_update(alphabet_param, total_freq);
        // 桁確定1
        // 上位8bitが決定した場合
        // 8bit左シフトを行い、決定した桁を取り出しておく
        const TOP8: u64 = 1 << (64 - 8);
        while (self.range_coder.lower_bound())
            ^ (self.range_coder.range() + self.range_coder.lower_bound())
            < TOP8
        {
            // println!("桁確定");
            self.data.push_back(self.range_coder.left_shift());
            put_byte += 1
        }
        // 桁確定2
        // レンジが小さくなったら
        // 次の上位8bitが変動しないギリギリまで範囲を絞り出力する
        const TOP16: u64 = 1 << (64 - 16);
        while self.range_coder.range() < TOP16 {
            // println!("範囲を絞る");
            let range_new = !self.range_coder.lower_bound() & (TOP16 - 1);
            self.range_coder.set_range(range_new);
            self.data.push_back(self.range_coder.left_shift());
            put_byte += 1;
        }
        /*
        println!();
        println!("lobo:0x{:x}", self.range_coder.lower_bound());
        println!("rage:0x{:x}", self.range_coder.range());
        println!();
        */
        //1アルファベットエンコード完了
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
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            range_coder: RangeCoder::new(),
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
    pub fn set_range_coder(&mut self, rangecoder: RangeCoder) {
        self.range_coder = rangecoder;
    }
}
