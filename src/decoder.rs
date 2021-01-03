//! デコーダ
use crate::pmodel::PModel;
use crate::range_coder::RangeCoder;
use std::collections::VecDeque;
/// デコーダ構造体
pub struct Decoder {
    range_coder: RangeCoder,
    // 符号のうち，未使用のものをバッファしておく
    buffer: VecDeque<u8>,
    // 符号のうち，レンジコーダの下限と同じ位置のビット列
    data: u64,
}
impl Decoder {
    pub fn new<I: Into<VecDeque<u8>>>(code: I) -> Self {
        let mut decoder = Self {
            range_coder: RangeCoder::new(),
            buffer: code.into(),
            data: 0,
        };
        // 最初の64bit読み出し
        decoder.shift_left_buffer(8);
        decoder
    }
    pub fn range_coder(&self) -> &RangeCoder {
        &self.range_coder
    }
    pub fn data(&self) -> u64 {
        self.data
    }
    /// dataをn回左シフトして、バッファからデータを入れる
    fn shift_left_buffer(&mut self, n: usize) {
        for _ in 0..n {
            self.data = (self.data << 8) | self.buffer.pop_front().unwrap() as u64;
        }
    }

    /// 一文字デコードする関数
    pub fn decode<T: PModel>(&mut self, pmodel: &T) -> usize {
        // デコードするアルファベットのインデックスをとってくる
        let decode_index = pmodel.find_index(self);
        // レンジコーダの状態の更新
        let n = self
            .range_coder
            .param_update(
                pmodel.c_freq(decode_index),
                pmodel.cum_freq(decode_index),
                pmodel.total_freq(),
            )
            .unwrap()
            .len();
        self.shift_left_buffer(n);
        decode_index
    }
}
