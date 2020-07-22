//! エンコーダ
use crate::pmodel::PModel;
use crate::range_coder::RangeCoder;
use std::collections::VecDeque;

/// エンコーダ構造体
pub struct Encoder {
    /// レンジコーダ
    range_coder: RangeCoder,
    /// 出力する符号
    data: VecDeque<u8>,
}
/// ロジック
impl Encoder {
    /// 1アルファベット、エンコードを進める
    ///
    /// 返値は出力したバイト数
    pub fn encode<T: PModel>(&mut self, pmodel: &T, index: usize) -> u32 {
        // 下限、レンジの更新
        let mut out = self
            .range_coder
            .param_update(
                pmodel.c_freq(index),
                pmodel.cum_freq(index),
                pmodel.total_freq(),
            )
            .unwrap();
        let len = out.len();
        self.data.append(&mut out);
        len as u32
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
impl Default for Encoder {
    fn default() -> Self {
        Self {
            data: VecDeque::new(),
            range_coder: RangeCoder::new(),
        }
    }
}
/// コンストラクタ
impl Encoder {
    pub fn new() -> Self {
        Encoder::default()
    }
}
/// ゲッタ
impl Encoder {
    pub fn data(&self) -> &VecDeque<u8> {
        &self.data
    }
    pub fn range_coder(&self) -> &RangeCoder {
        &self.range_coder
    }
}
