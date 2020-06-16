//! デコーダ
use crate::pmodel::PModel;
use crate::range_coder::RangeCoder;
use std::collections::VecDeque;
/// デコーダ構造体
pub struct Decoder {
    // エンコーダの動作を再現するためのエンコーダ構造体
    range_coder: RangeCoder,
    // エンコーダの出力を入れる
    buffer: VecDeque<u8>,
    // bufferから順に読み出して使う
    data: u64,
}
impl Default for Decoder {
    fn default() -> Self {
        Self {
            range_coder: RangeCoder::new(),
            buffer: VecDeque::new(),
            data: 0,
        }
    }
}
/// コンストラクタ,セッター,ゲッター
impl Decoder {
    /// コンストラクタ
    pub fn new() -> Self {
        Decoder::default()
    }
    pub fn set_data(&mut self, data: VecDeque<u8>) {
        self.buffer = data;
    }
    pub fn set_rangecoder(&mut self, range_coder: RangeCoder) {
        self.range_coder = range_coder;
    }
    pub fn range_coder(&self) -> &RangeCoder {
        &self.range_coder
    }
    pub fn range_coder_mut(&mut self) -> &mut RangeCoder {
        &mut self.range_coder
    }
    pub fn buffer(&self) -> &VecDeque<u8> {
        &self.buffer
    }
    pub fn data(&self) -> u64 {
        self.data
    }
}
/// ロジック
impl Decoder {
    /// デコード開始用の関数
    pub fn decode_start(&mut self) {
        // 最初の64bit読み出し
        self.shift_left_buffer(8);
    }
    /// dataをn回左シフトして、バッファからデータを入れる
    fn shift_left_buffer(&mut self, n: usize) {
        for _ in 0..n {
            self.data = (self.data << 8) | self.buffer.pop_front().unwrap() as u64;
        }
    }
    /// 一文字デコードする関数
    pub fn decode_one_alphabet<T: PModel>(&mut self, pmodel: &T) -> usize {
        // デコードするアルファベットのインデックスをとってくる
        let decode_index = pmodel.find_index(self);
        // println!("alphabet index is: {}", decode_index);
        // エンコーダの状態の更新
        let n = self
            .range_coder_mut()
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
