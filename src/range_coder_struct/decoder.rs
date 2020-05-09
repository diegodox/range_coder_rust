//! デコードする時に使う
//!
use crate::range_coder_struct::RangeCoder;
use crate::simbol_data::Simbols;
use crate::simbol_data::MAX_SIMBOL_COUNT;
use crate::uext::UEXT;
use std::fs::File;
use std::io::Read;
use std::path::Path;

impl RangeCoder {
    /// ファイル読み込み
    ///
    /// データ構造(これは違う)
    /// 名前|先頭バイト|形式
    /// -|-|-
    /// シンボルの種類数|0|u8
    /// シンボルデータ|1|シンボルそのもの(サイズは外部指定)、シンボルの出現数(u32)
    /// 符号|$(size[byte]+4)\times+1$|符号
    pub fn read(path: &Path) -> Result<RangeCoder, String> {
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
        Result::Ok(rc)
    }
    pub fn decode(mut self) -> Vec<usize> {
        let mut decoded_simbol = Vec::new();
        let mut shift_count = 0;
        let simbol_total = self.simbol_total();
        for _ in 0..simbol_total {
            decoded_simbol.push(self.decode_one_simbol(&mut shift_count));
        }
        decoded_simbol.reverse();
        decoded_simbol
    }
    /// 一文字デコードする関数
    fn decode_one_simbol(&mut self, shift_count: &mut u32) -> usize {
        // 符号の復元(8bit区切りから32ビットへ)
        let mut v: u32 = 0;
        for i in 0..4 {
            v |= (self.data[i + *shift_count as usize] as u32) << 8 * (3 - i);
        }
        //println!("v:{:x}", v);
        // 力技で行う
        // うまい方法はあとで考える
        let range_before = self.range / self.simbol_data.total as u32;
        let mut decode_index = 0;
        for try_index in 0..MAX_SIMBOL_COUNT {
            let simbol_data = self.simbol_data.simbol_param(try_index);
            // Rangeの更新
            let range_try = match (simbol_data.cum + simbol_data.c).cmp(&self.simbol_data.total) {
                // レンジ最後のシンボルの場合、通常のレンジ更新で発生する誤差(整数除算によるもの)を含める
                std::cmp::Ordering::Equal => self.range - range_before * simbol_data.cum,
                // レンジ最後のシンボルでない場合、通常のレンジ更新を行う
                std::cmp::Ordering::Less => range_before * simbol_data.c,
                // Graterになることはない
                _ => unreachable!("panic! (cum+c) should not be bigger than total"),
            };
            // lower_boundの更新
            let lower_bound_try = match self
                .lower_bound
                .overflowing_add(range_before * simbol_data.cum)
            {
                (v, _bool) => v,
            };
            //println!("l_v_try  : {:x}", lower_bound_try);
            //println!("ragne_try: {:x}", range_try);
            if v >= lower_bound_try {
                if v - lower_bound_try < range_try {
                    decode_index = try_index;
                    break;
                }
            }
            //println!("index: {} fail.", try_index);
        }
        //println!("インデックス: {}", decode_index);

        // decode_indexをunmutableに
        let decode_index = decode_index;
        /*
        以下、エンコードの再現
        */
        // decode_indexのシンボルデータの取得
        let simbol_data = self.simbol_data.simbol_param(decode_index);
        // Range/totalの一時保存
        let range_before = self.range / self.simbol_data.total as u32;
        // Rangeの更新
        match (simbol_data.cum + simbol_data.c).cmp(&self.simbol_data.total) {
            // レンジ最後のシンボルの場合、通常のレンジ更新で発生する誤差(整数除算によるもの)を含める
            std::cmp::Ordering::Equal => {
                self.range = self.range - range_before * simbol_data.cum;
            }
            // レンジ最後のシンボルでない場合、通常のレンジ更新を行う
            std::cmp::Ordering::Less => {
                self.range = range_before * simbol_data.c;
            }
            // Graterになることはない
            _ => unreachable!("panic! (cum+c) should not be bigger than total"),
        }
        // lower_boundの更新
        match self
            .lower_bound
            .overflowing_add(range_before * simbol_data.cum)
        {
            (v, true) => {
                self.lower_bound = v;
            }
            (v, false) => {
                self.lower_bound = v;
            }
        }
        /*
        上位8bitの判定
        */
        static TOP: u32 = 1 << 24;
        while self.range < TOP {
            //println!("overflow");
            *shift_count += 1;
            self.lower_bound <<= 8;
            self.range <<= 8;
        }
        decode_index
    }
}
