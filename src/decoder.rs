//! デコーダ
//!
//! 動かない
use crate::range_coder_struct::RangeCoder;
use crate::simbol_data::Simbols;
use crate::simbol_data::MAX_SIMBOL_COUNT;
use crate::uext::UEXT;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Decoder {
    range_coder: RangeCoder,
    // エンコーダの出力を入れる
    buffer: VecDeque<u8>,
    // bufferから順に読み出して使う
    data: u32,
}
//折り畳みを容易にするためのimpl分割
impl Decoder {
    pub fn new(range_coder: RangeCoder) -> Self {
        Self {
            range_coder: range_coder,
            buffer: VecDeque::new(),
            data: 0,
        }
    }
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
}
impl Decoder {
    pub fn decode(mut self) -> Vec<usize> {
        let mut decoded_simbol = Vec::new();
        let simbol_total = self.range_coder.simbol_total();
        let mut data_buf = Vec::new();
        println!("buffer length: {}", self.buffer.len());
        // 最初の32bit読み出し
        for _ in 0..4 {
            match self.buffer.pop_front() {
                Some(v) => data_buf.push(v),
                None => {}
            }
        }
        self.data = UEXT::from_vec_u8(&data_buf);
        // シンボル数分デコード
        for _ in 0..simbol_total {
            decoded_simbol.push(self.decode_one_simbol());
        }
        decoded_simbol
    }
    /// シンボルを見つける関数
    fn find_simbol(&self) -> usize {
        let mut left = 0;
        let mut right = MAX_SIMBOL_COUNT - 1;
        loop {
            let try_index = (left + right) / 2;
            let simbol_data = self.range_coder.simbol_data().simbol_param(try_index);
            // Rangeの更新
            let range_try = self.range_coder.range_when_encode(simbol_data);
            // lower_boundの更新
            let (lower_bound_try, _is_overflow) =
                self.range_coder.lower_bound_when_encode(simbol_data);
            /*
            println!(
                "try index is      : ( {}+{} ) /2 = {}",
                left, right, try_index
            );
            println!("下限 try          : {:x},{:?}", lower_bound_try, of);
            println!("レンジ try        : {:x}", range_try);
            println!(
                "下限+レンジ try   : {:x}",
                range_try as u64 + lower_bound_try as u64
            );
            */
            if range_try.overflowing_add(lower_bound_try).1 {
                match range_try.overflowing_add(lower_bound_try).0.cmp(&self.data) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                        //println!("try bigger");
                        left = try_index + 1;
                    }
                    std::cmp::Ordering::Greater => {
                        return try_index;
                    }
                }
            } else {
                match lower_bound_try.cmp(&self.data) {
                    std::cmp::Ordering::Greater => {
                        if (left == right) && (try_index == 0) && (left == 0) {
                            panic!("something wrong!");
                        }
                        //println!("try smaller");
                        //println!();
                        right = try_index;
                    }
                    _ => match (range_try + lower_bound_try).cmp(&self.data) {
                        std::cmp::Ordering::Greater => {
                            return try_index;
                        }
                        _ => {
                            //println!("try bigger");
                            //println!();
                            left = try_index + 1;
                        }
                    },
                }
            }
        }
    }
    /// 一文字デコードする関数
    fn decode_one_simbol(&mut self) -> usize {
        /*println!("開始時のレンジコーダの状態");
        println!("下限  :0x{:x}", self.range_coder.lower_bound());
        println!("レンジ:0x{:x}", self.range_coder.range());
        println!("データ:0x{:x}", self.data);
        */
        // シンボルを見つける
        let decode_index = self.find_simbol();
        println!("simbol is: {}", decode_index);
        // シンボルのパラメータを保存
        let decode_param = self.range_coder.simbol_data().simbol_param(decode_index);
        // range,lower_boundの更新
        let range_new = self.range_coder.range_when_encode(decode_param);
        let lower_bound_new = self.range_coder.lower_bound_when_encode(decode_param).0;
        self.range_coder.set_range(range_new);
        self.range_coder.set_lower_bound(lower_bound_new);
        // 下限をbitシフトしたタイミングで読み出す桁を変えればよい
        static TOP: u32 = 1 << 24;
        while self.range_coder.range() < TOP {
            //println!("data shift!");
            self.range_coder
                .set_lower_bound(self.range_coder.lower_bound() << 8);
            self.range_coder.set_range(self.range_coder.range() << 8);
            self.data_shift();
        }
        decode_index
    }
    fn data_shift(&mut self) {
        self.data <<= 8;
        match self.buffer.pop_front() {
            Some(v) => self.data |= v as u32,
            None => {}
        }
    }
}
