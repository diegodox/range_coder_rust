//! デコードする時に使う
//!
use crate::range_coder_struct::RangeCoder;
use crate::simbol_data::Simbols;
use crate::simbol_data::MAX_SIMBOL_COUNT;
use crate::uext::UEXT;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Decoder {
    range_coder: RangeCoder,
    // エンコーダの出力を入れる
    buffer: Vec<u8>,
    // bufferから順に読み出して使う
    data: u32,
}
impl RangeCoder {
    pub fn into_decoder(self) -> Decoder {
        Decoder {
            range_coder: self,
            buffer: Vec::new(),
            data: 0,
        }
    }
}
impl Decoder {
    /// ファイル読み込み
    ///
    /// データ構造(これは違う)
    /// 名前|先頭バイト|形式
    /// -|-|-
    /// シンボルの種類数|0|u8
    /// シンボルデータ|1|シンボルそのもの(サイズは外部指定)、シンボルの出現数(u32)
    /// 符号|$(size[byte]+4)\times+1$|符号
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
            sd.simbol_paramaters[index_buff].c = c;
        }
        sd.finalize();
        // シンボルデータからレンジコーダ作成
        let mut rc = RangeCoder::new(sd);
        // 出力データ読み込み
        rc.data = (&buff[cursor..]).iter().map(|x| *x).collect();
        let decoder = rc.into_decoder();
        Result::Ok(decoder)
    }
    pub fn decode(mut self) -> Vec<usize> {
        let mut decoded_simbol = Vec::new();
        let simbol_total = self.range_coder.simbol_total();
        self.buffer.reverse();
        let mut data_buf = Vec::new();
        // 初期のデータ32bit読み出し
        for _ in 0..4 {
            data_buf.push(self.buffer.pop().unwrap());
        }
        self.data = UEXT::from_vec_u8(&data_buf);
        // シンボル数分デコード
        for _ in 0..simbol_total {
            decoded_simbol.push(self.decode_one_simbol());
        }
        decoded_simbol.reverse();
        decoded_simbol
    }
    /// 一文字デコードする関数
    fn decode_one_simbol(&mut self) -> usize {
        let range_before = self.range_coder.range / self.range_coder.simbol_data.total as u32;
        let mut decode_index = 0;
        let mut left = 0;
        let mut right = MAX_SIMBOL_COUNT;
        while left < right {
            let try_index = (left + right) / 2;
            let simbol_data = self.range_coder.simbol_data.simbol_param(try_index);
            // Rangeの更新
            let range_try =
                match (simbol_data.cum + simbol_data.c).cmp(&self.range_coder.simbol_data.total) {
                    // レンジ最後のシンボルの場合、通常のレンジ更新で発生する誤差(整数除算によるもの)を含める
                    std::cmp::Ordering::Equal => {
                        self.range_coder.range - range_before * simbol_data.cum
                    }
                    // レンジ最後のシンボルでない場合、通常のレンジ更新を行う
                    std::cmp::Ordering::Less => range_before * simbol_data.c,
                    // Graterになることはない
                    _ => unreachable!("panic! (cum+c) should not be bigger than total"),
                };
            // lower_boundの更新
            let lower_bound_try = match self
                .range_coder
                .lower_bound
                .overflowing_add(range_before * simbol_data.cum)
            {
                (v, _bool) => v,
            };
            match self.data >= lower_bound_try {
                // 下限以上
                true => match self.data - lower_bound_try < range_try {
                    // 条件ピッタリ
                    true => {
                        decode_index = try_index;
                        break;
                    }
                    // もっと前のシンボル
                    false => {
                        right = try_index;
                    }
                },
                // もっと後のシンボル
                false => {
                    left = try_index + 1;
                }
            }
        }

        // decode_indexをunmutableに
        let decode_index = decode_index;
        /*
        以下、エンコードの再現
        */
        // decode_indexのシンボルデータの取得
        let simbol_data = self.range_coder.simbol_data.simbol_param(decode_index);
        // Range/totalの一時保存
        let range_before = self.range_coder.range / self.range_coder.simbol_data.total as u32;
        // Rangeの更新
        match (simbol_data.cum + simbol_data.c).cmp(&self.range_coder.simbol_data.total) {
            // レンジ最後のシンボルの場合、通常のレンジ更新で発生する誤差(整数除算によるもの)を含める
            std::cmp::Ordering::Equal => {
                self.range_coder.range = self.range_coder.range - range_before * simbol_data.cum;
            }
            // レンジ最後のシンボルでない場合、通常のレンジ更新を行う
            std::cmp::Ordering::Less => {
                self.range_coder.range = range_before * simbol_data.c;
            }
            // Graterになることはない
            _ => unreachable!("panic! (cum+c) should not be bigger than total"),
        }
        // lower_boundの更新
        match self
            .range_coder
            .lower_bound
            .overflowing_add(range_before * simbol_data.cum)
        {
            (v, true) => {
                self.range_coder.lower_bound = v;
            }
            (v, false) => {
                self.range_coder.lower_bound = v;
            }
        }
        /*
        上位8bitの判定
        */
        static TOP: u32 = 1 << 24;
        while self.range_coder.range < TOP {
            self.range_coder.lower_bound <<= 8;
            self.range_coder.range <<= 8;
            self.data <<= 8;
            self.data |= self.buffer.pop().unwrap() as u32;
        }
        decode_index
    }
}
