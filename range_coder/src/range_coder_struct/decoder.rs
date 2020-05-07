use crate::range_coder_struct::RangeCoder;
use crate::simbol_data::SimbolParam;
use crate::simbol_data::Simbols;
use crate::simbol_trait::ForRangeCoder;
use crate::uext::UEXT;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// ファイル読み込み
///
/// データ構造
/// 名前|先頭バイト|形式
/// -|-|-
/// シンボルの種類数|0|u8
/// シンボルデータ|1|シンボルそのもの(サイズは外部指定)、シンボルの出現数(u32)
/// 符号|$(size[byte]+4)\times+1$|符号

impl<T> RangeCoder<T>
where
    T: Eq + std::hash::Hash + ForRangeCoder + Ord + std::fmt::Debug,
{
    pub fn read(path: &Path) -> Result<RangeCoder<T>, String>
    where
        T: Eq + std::hash::Hash + ForRangeCoder + Ord,
    {
        // ファイルオープン
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("file could not open".to_string()),
        };
        // ファイル読み込み
        let mut buff = Vec::new();
        let mut cursor = 0;
        file.read_to_end(&mut buff).unwrap();
        // シンボルデータ部分読み込み
        // シンボル構造体作成
        let mut sd = Simbols::new(T::size());
        // シンボル数読み込み
        let count = &buff[0..4];
        cursor += 4;
        let mut sc = [0; 4];
        (&count[..]).read_exact(&mut sc).unwrap();
        sd.simbol_type_count = u32::from_be_bytes(sc);
        //println!("シンボル数:{}", sd.simbol_type_count);
        // シンボル読み込み
        for i in 0..sd.simbol_type_count {
            // simbol分切り出し
            let simbol_buff = &buff[cursor..cursor + sd.size as usize];
            cursor += sd.size as usize;
            // c分切り出し
            let c_buff = &buff[cursor..cursor + SimbolParam::size() as usize];
            cursor += SimbolParam::size() as usize;
            let simbol: T = ForRangeCoder::read(simbol_buff);
            //println!("シンボルデータ:{:?}", simbol);
            let c: u32 = UEXT::from_vec_u8(c_buff);
            //println!("シンボルの出現回数:{}", c);
            sd.index.entry(simbol).or_insert(i);
            sd.simbol_paramaters.push_back(SimbolParam::new_with_c(c));
        }
        sd.finalize();
        // シンボルデータからレンジコーダ作成
        let mut rc: RangeCoder<T> = RangeCoder::new(sd);
        // 出力データ読み込み
        rc.data = (&buff[cursor..]).iter().map(|x| *x).collect();
        Result::Ok(rc)
    }
    /// 一文字デコードする関数
    pub fn decode(&mut self, shift_count: &mut u8) -> &T {
        // 符号の復元
        let mut v: u32 = 0;
        for i in 0..4 {
            v |= (self.data[i + *shift_count as usize] as u32) << 8 * (3 - i);
        }
        println!("v:{:x}", v);
        // 力技で行う
        // うまい方法はあとで考える
        let range_before = self.range / self.simbol_data.total as u32;
        let mut decode_index = 0;
        for try_index in 0..self.simbol_data.simbol_type_count {
            let mut range_try = 0;
            let mut lower_bound_try = 0;
            let simbol_data = self.simbol_data.get_by_index(try_index).unwrap();
            // Rangeの更新
            match (simbol_data.cum + simbol_data.c).cmp(&self.simbol_data.total) {
                // レンジ最後のシンボルの場合、通常のレンジ更新で発生する誤差(整数除算によるもの)を含める
                std::cmp::Ordering::Equal => {
                    range_try = self.range - range_before * simbol_data.cum;
                }
                // レンジ最後のシンボルでない場合、通常のレンジ更新を行う
                std::cmp::Ordering::Less => {
                    range_try = range_before * simbol_data.c;
                }
                // Graterになることはない
                _ => unreachable!("panic! (cum+c) should not be bigger than total"),
            }
            // lower_boundの更新
            match self
                .lower_bound
                .overflowing_add(range_before * simbol_data.cum)
            {
                (v, _bool) => {
                    lower_bound_try = v;
                }
            }
            println!("l_v_try  : {:x}", lower_bound_try);
            println!("ragne_try: {:x}", range_try);
            if v >= lower_bound_try {
                if v - lower_bound_try < range_try {
                    decode_index = try_index;
                    break;
                }
            }
            println!("index: {} fail.", try_index);
        }
        println!("インデックス: {}", decode_index);

        // インデックスからシンボルを探す
        let decoded_simbol = self
            .simbol_data
            .index
            .iter()
            .filter(|(_k, &v)| v == decode_index as u32)
            .map(|(k, _v)| k)
            .last()
            .unwrap();

        /*
        以下、エンコードの再現
        */
        // simbolのindexをとる
        let simbol_data = self.simbol_data.get(decoded_simbol).unwrap();
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
            println!("overflow");
            *shift_count += 1;
            self.lower_bound <<= 8;
            self.range <<= 8;
        }
        decoded_simbol
    }
}
