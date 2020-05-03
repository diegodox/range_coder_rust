//! This module is range_coder.
//!
//! 参考文献:[CodeZine: 高速な算術符号を実現する「Range Coder」](https://codezine.jp/article/detail/443)
use std::collections::HashMap;
use std::collections::VecDeque;
use std::u32;

/// **RangeCoder構造体**
///
/// RangeCoder<シンボルのデータ型>で指定
pub struct RangeCoder<T>
where
    T: Eq + std::hash::Hash,
{
    /// 符号
    data: VecDeque<u8>,
    /// 下限
    lower_bound: u32,
    /// 幅
    range: u32,
    /// 全文字の出現回数
    total: u32,
    /// インデックスと文字の対応
    index: HashMap<T, SimbolData>,
    /// バッファ
    buffer: u8,
    /// 0xff or 0x00 になる値の個数
    /// (参考文献でcarryNと呼ばれるもの)
    carry_n: usize,
    /// range < 1<<24 になったことがあるかどうか
    /// true  : ない
    /// false : ある
    is_start: bool,
}

pub struct SimbolData {
    /// 文字の累積出現回数
    cum: u32,
    /// 文字の出現回数
    c: u32,
}
impl SimbolData {
    fn new() -> Self {
        SimbolData { cum: 0, c: 0 }
    }
}
/// ファイルから符号化するための情報を作る
/// シンボルの種類とデータを渡す
// #[TODO]実装

impl<T> RangeCoder<T>
where
    T: Eq + std::hash::Hash,
{
    pub fn pr(&self) {
        println!("   ENCODER STATE");
        print!("      data :");
        for i in &(self.data) {
            print!("0x{:x} , ", i);
        }
        println!("");
        println!("      lower_bound :0x{:x}", &(self.lower_bound));
        println!("      range       :0x{:x}", &(self.range));
    }
    pub fn new() -> Self {
        RangeCoder {
            data: VecDeque::new(),
            lower_bound: 0,
            range: u32::MAX,
            total: 0,
            index: HashMap::new(),
            buffer: 0,
            carry_n: 0,
            is_start: true,
        }
    }
    /// シンボルを追加
    /// シンボルの追加が終わったら reflesh_cum()を呼ぶこと
    pub fn add_simbol(&mut self, simbol: T) {
        self.index.entry(simbol).or_insert(SimbolData::new()).c += 1;
    }
    /// 出現回数cの変更を累積出現回数cumに反映
    pub fn reflesh_cum(&mut self) {
        let mut total = 0;
        let mut vec = Vec::new();
        for (key, simbol_data) in &mut self.index {
            vec.push((key, simbol_data));
        }
        vec.sort_by(|a, b| a.1.c.cmp(&(b.1.c)));
        let mut hash = HashMap::new();
        for mut i in vec {
            i.1.cum = total;
            total += i.1.c;
            hash.entry(i.0).or_insert(i.1);
        }
        self.total = total;
    }
    /// 1シンボル、エンコードを進める
    pub fn encode(&mut self, simbol: T) {
        println!("エンコード開始時点のエンコーダの状態");
        self.pr();

        // simbolのindexをとる
        let simbol_data = self.index.get(&simbol).unwrap();
        // Range/totalの一時保存
        let range_before = self.range / self.total as u32;
        //println!("   range(before)/total: {:x}", range_before);

        // Rangeの更新
        println!("   Rangeの更新");
        match (simbol_data.cum + simbol_data.c).cmp(&self.total) {
            // レンジ最後のシンボルの場合、通常のレンジ更新で発生する誤差(整数除算によるもの)を含める
            std::cmp::Ordering::Equal => {
                println!("   last simbol found");
                self.range = self.range - range_before * simbol_data.cum;
                // println!(
                //     "   更新後のRange: 0x{:x} = Range(0x{:x}) - range_before/total(0x{:x}) * simbol's cum(0x{:x})",
                //     self.range, tmp, range_before,simbol_data.cum,
                // );
            }
            // レンジ最後のシンボルでない場合、通常のレンジ更新を行う
            std::cmp::Ordering::Less => {
                //let tmp = self.range;
                // println!("   not last simbol found");
                self.range = range_before * simbol_data.c;
                // println!(
                //     "   更新後のRange: {:x} = range_before/total(0x{:x}) * simbol_data.cum(0x{:x})",
                //     self.range, tmp, simbol_data.cum
                // );
            }
            // Graterになることはない
            _ => unreachable!("panic! (cum+c) should not be bigger than total"),
        }
        // lower_boundの更新
        println!("   下限の更新");
        match self
            .lower_bound
            .overflowing_add(range_before * simbol_data.c)
        {
            (v, true) => {
                // lower_boundがオーバーフローした場合
                // 出力待ち桁はbuffer+1 00 00 00 00 .. 00に確定する。
                println!("   overflow occured");
                self.buffer += 1;
                while self.carry_n > 0 {
                    // buffer+1 00 00 .. 00の出力
                    self.data.push_back(self.buffer);
                    for _ in 0..self.carry_n {
                        self.data.push_back(0x00);
                    }
                    self.carry_n = 0;
                    self.buffer = 0;
                }
                //println!("   更新後の下限: {:x}", v);
                self.lower_bound = v;
            }
            (v, false) => {
                //println!("   overflow NOT occured");
                //println!("   更新後の下限: {:x}", v);
                self.lower_bound = v;
            }
        }
        //println!("   上位8bitの判定");
        // 上位8bitが決定した場合、シフトを行い、決定した桁を取り出しておく
        static TOP: u32 = 1 << 24;
        while self.range < TOP {
            self.pr();
            //println!("   上位8bitは確定");
            //println!("   出力処理を開始");
            // 確定した上位8bit(下限の上位8bit)をbuffer_newに格納
            let buffer_new = (self.lower_bound >> 24) as u8;
            if self.is_start == true {
                self.buffer = buffer_new;
                self.is_start = false;
            } else {
                match buffer_new {
                    // バッファが1111 1111だった場合
                    0xff => {
                        self.carry_n += 1;
                    }
                    // バッファが普通の値だった場合
                    _ => {
                        // #buffer ff ff ff..ffの出力
                        self.data.push_back(self.buffer);
                        for _ in 0..self.carry_n {
                            self.data.push_back(0xff);
                        }
                        self.carry_n = 0;
                        self.buffer = buffer_new;
                    }
                }
            }
            println!("   8bit shift left");
            // 先頭8bitはバッファに入れたのでシフトして演算精度をあげる
            self.lower_bound <<= 8;
            self.range <<= 8;
            //self.pr();
        }
        println!("   先頭は未決定");
        println!("   一文字エンコード完了");
    }
    /// エンコード終了後に呼び出して、
    /// buffer,carry_nを出力する。
    pub fn finish(&mut self) {
        self.data.push_back(self.buffer);
        for _ in 0..self.carry_n {
            self.data.push_back(0xff);
        }
        self.data.push_back((self.lower_bound >> 24) as u8);
        self.lower_bound <<= 8;
        self.data.push_back((self.lower_bound >> 24) as u8);
        self.lower_bound <<= 8;
        self.data.push_back((self.lower_bound >> 24) as u8);
        self.lower_bound <<= 8;
        self.data.push_back((self.lower_bound >> 24) as u8);
    }
}
