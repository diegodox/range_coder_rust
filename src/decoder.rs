//! デコーダ
use crate::pmodel::PModel;
use crate::range_coder::RangeCoder;
use std::collections::VecDeque;
/// デコーダ構造体
pub struct Decoder {
    range_coder: RangeCoder,
    // レンジコーダの下限と同じ位置の符号
    data: u64,
    // data以降の符号のバッファ
    buffer: VecDeque<u8>,
}
impl Decoder {
    pub fn new<I: Into<VecDeque<u8>>>(code: I) -> Self {
        let mut decoder = Self {
            range_coder: RangeCoder::new(),
            data: 0,
            buffer: code.into(),
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
        // デコーダ内部のdataを内部のレンジコーダと同期する
        self.shift_left_buffer(n);
        decode_index
    }
}
