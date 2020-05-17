//! デコーダ
use crate::encoder::Encoder;
use crate::freq_table::FreqTable;
use std::collections::VecDeque;

pub struct Decoder {
    // エンコーダの動作を再現するためのエンコーダ構造体
    encoder: Encoder,
    // エンコーダの出力を入れる
    buffer: VecDeque<u8>,
    // bufferから順に読み出して使う
    data: u64,
}
//折り畳みを容易にするためのimpl分割
impl Decoder {
    pub fn new() -> Self {
        Self {
            encoder: Encoder::new(),
            buffer: VecDeque::new(),
            data: 0,
        }
    }
    pub fn set_data(&mut self, data: VecDeque<u8>) {
        self.buffer = data;
    }
    /// dataをn回左シフトして、バッファからデータを入れる
    fn shift_left_buffer(&mut self, n: u32) {
        for _ in 0..n {
            self.data = (self.data << 8) | self.buffer.pop_front().unwrap() as u64;
        }
    }
    pub fn set_encoder(&mut self, encoder: Encoder) {
        self.encoder = encoder;
    }
}
impl Decoder {
    // デコード開始用の関数
    pub fn decode_start(&mut self) {
        println!("buffer length: {}", self.buffer.len());
        // 最初の64bit読み出し
        self.shift_left_buffer(8);
    }
    /// アルファベットを見つける関数
    fn find_alphabet(&self, freq_table: &FreqTable) -> usize {
        let mut left = 0;
        let mut right = freq_table.alphabet_count() - 1;
        let rfreq = (self.data - self.encoder.range_coder().lower_bound())
            / self
                .encoder
                .range_coder()
                .range_par_total(freq_table.total_freq());
        /*
        println!();
        println!("data=0x{:x}", self.data);
        println!("lobo=0x{:x}", self.encoder.range_coder().lower_bound());
        println!(
            "da-l=0x{:x}",
            self.data - self.encoder.range_coder().lower_bound()
        );
        println!(
            "r/to=0x{:x}",
            self.encoder
                .range_coder()
                .range_par_total(freq_table.total_freq())
        );
        println!("rage=0x{:x}", self.encoder.range_coder().range());
        println!("totl={}", freq_table.total_freq());
        println!();
        println!("target_freq={}", rfreq);
        */
        while left < right {
            let mid = (left + right) / 2;
            let mid_param = freq_table.alphabet_param(mid + 1);
            /*
            println!("mid_index:{}", mid);
            println!("mid+1 param c:{},cum:{}", mid_param.c(), mid_param.cum());
            */
            if mid_param.cum() as u64 <= rfreq {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
    /// 一文字デコードする関数
    pub fn decode_one_alphabet(&mut self, freq_table: &FreqTable) -> usize {
        // アルファベットを見つける
        let decode_index = self.find_alphabet(freq_table);
        // println!("alphabet index is: {}", decode_index);
        // エンコーダの状態の更新
        let n = self.encoder.encode(
            freq_table.alphabet_param(decode_index),
            freq_table.total_freq(),
        );
        self.shift_left_buffer(n);
        decode_index
    }
}
