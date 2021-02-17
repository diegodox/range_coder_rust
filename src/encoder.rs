//! エンコーダ
use crate::pmodel::PModel;
use crate::range_coder::RangeCoder;
use std::collections::VecDeque;

/// エンコーダ構造体
pub struct Encoder {
    pub range_coder: RangeCoder,
    /// 出力する符号
    code: VecDeque<u8>,
}
/// ロジック
impl Encoder {
    pub fn new() -> Self {
        Encoder::default()
    }
    /// take ref to code
    pub fn peek_code(&self) -> &VecDeque<u8> {
        &self.code
    }
    /// 1アルファベット、エンコードを進める
    ///
    /// 返値は出力したバイト数
    pub fn encode<T: PModel>(&mut self, pmodel: &T, index: usize) -> u32 {
        // 下限、レンジの更新
        let mut outbytes = self
            .range_coder
            .param_update(
                pmodel.c_freq(index),
                pmodel.cum_freq(index),
                pmodel.total_freq(),
            )
            .unwrap();
        let len = outbytes.len();
        self.code.append(&mut outbytes);
        len as u32
    }
    /// エンコード終了処理
    /// 下限を符号に出力し，符号を返す
    pub fn finish(mut self) -> VecDeque<u8> {
        // 現状の下限を出力
        for _ in 0..8 {
            self.code.push_back(self.range_coder.left_shift());
        }
        self.code
    }
}
impl Default for Encoder {
    fn default() -> Self {
        Self {
            code: VecDeque::new(),
            range_coder: RangeCoder::new(),
        }
    }
}
