//! デコーダ
//!

use crate::encoder::Encoder;
use crate::simbol_data::Simbols;
use crate::simbol_data::MAX_SIMBOL_COUNT;
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
    /// シンボルを見つける関数
    fn find_simbol(&self, simbols: &Simbols) -> usize {
        let mut left = 0;
        let mut right = MAX_SIMBOL_COUNT - 1;
        let rfreq = (self.data - self.encoder.range_coder().lower_bound())
            / self
                .encoder
                .range_coder()
                .range_par_total(simbols.total_freq());
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
                .range_par_total(simbols.total_freq())
        );
        println!("rage=0x{:x}", self.encoder.range_coder().range());
        println!("totl={}", simbols.total_freq());
        println!();
        println!("target_freq={}", rfreq);
        */
        while left < right {
            let mid = (left + right) / 2;
            let mid_param = simbols.simbol_param(mid + 1);
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
    pub fn decode_one_simbol(&mut self, simbols: &Simbols) -> usize {
        // シンボルを見つける
        let decode_index = self.find_simbol(simbols);
        // println!("simbol is: {}", decode_index);
        // エンコーダの状態の更新
        let n = self
            .encoder
            .encode(simbols.simbol_param(decode_index), simbols.total_freq());
        self.shift_left_buffer(n);
        decode_index
    }
}

/*
pub fn read(path: &Path) -> Result<Decoder, String> {
    // ファイルオープン
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err("file could not open".to_string()),
    };
    // ファイル読み込み
    // ファイルを読み込むためのバッファ
    let mut buff = Vec::new();
    // ファイルの何バイトまで読み込んだかを示すcursor
    // seekが使えるようにラッパーするのが標準であったと思うので、そっちに書き換えるべき
    let mut cursor = 0;
    // ファイル読み込み
    file.read_to_end(&mut buff).unwrap();
    // シンボルデータ部分読み込み
    // シンボル構造体作成
    let mut sd = Simbols::new();
    // usizeの大きさ読み込み
    let size_of_usize = buff[0] as usize;
    cursor += 1;
    // シンボル数を読み込む
    let simbol_count: usize = UEXT::from_vec_u8(&buff[cursor..cursor + size_of_usize]);
    cursor += size_of_usize;
    // シンボル読み込み
    for _ in 0..simbol_count {
        // index分切り出し
        let index_buff: usize = UEXT::from_vec_u8(&buff[cursor..cursor + size_of_usize]);
        cursor += size_of_usize;
        // c分切り出し
        let c: u32 = UEXT::from_vec_u8(&buff[cursor..cursor + 4]);
        cursor += 4;
        // 配列の該当箇所にcを登録
        sd.simbol_param_mut(index_buff).set_c(c);
    }
    sd.finalize();
    // シンボルデータからデコーダ作成
    let mut decoder = RangeCoder::new(sd).into_decoder();
    // 出力データ読み込み
    decoder.buffer = (&buff[cursor..]).iter().map(|x| *x).collect();

    //println!("simbol data: {:?}", decoder.range_coder.simbol_data());
    Result::Ok(decoder)
}
*/
