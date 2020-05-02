//! This module is range_coder.
//!
//! 参考文献:[CodeZine: 高速な算術符号を実現する「Range Coder」](https://codezine.jp/article/detail/443)
use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec::Vec;

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
    index: HashMap<T, usize>,
    /// 文字の累積出現回数
    cum: Vec<u32>,
    /// 文字の出現回数
    c: Vec<u32>,
    /// バッファ
    buffer: u8,
    /// 0xff or 0x00 になる値の個数
    /// (参考文献でcarryNと呼ばれるもの)
    carry_n: usize,
    /// range < 1<<24 になったことがあるかどうか
    is_start: bool,
}
/// ファイルから符号化するための情報を作る
/// シンボルの種類とデータを渡す
// #[TODO] 実装
fn make_count_list() {}

impl<T> RangeCoder<T>
where
    T: Eq + std::hash::Hash,
{
    fn get_index(&self, simbol: &T) -> Option<&usize> {
        self.index.get(simbol)
    }
    /// 1シンボル、エンコードを進める
    pub fn encode(&mut self, simbol: T) {
        // Range/totalの一時保存
        let range_before = self.range / self.total;
        // Rangeの更新
        match (self.cum[*(self.get_index(&simbol).unwrap())]
            + self.c[*(self.get_index(&simbol).unwrap())])
        .cmp(&self.total)
        {
            // レンジ最後のシンボルの場合、通常のレンジ更新で発生する誤差(整数除算によるもの)を含める
            std::cmp::Ordering::Equal => {
                self.range =
                    self.range - range_before * self.cum[*(self.get_index(&simbol).unwrap())]
            }
            // レンジ最後のシンボルでない場合、通常のレンジ更新を行う
            std::cmp::Ordering::Less => {
                self.range = range_before * self.cum[*(self.get_index(&simbol).unwrap())]
            }
            // Graterになることはない
            _ => unreachable!("panic! (cum+c) should not be bigger than total"),
        }
        // lower_boundの更新
        let lower_bound_new =
            self.lower_bound + range_before * self.c[*(self.index.get(&simbol).unwrap())];
        if lower_bound_new < self.lower_bound {
            // lower_boundがオーバーフローした場合
            // 出力待ち桁はbuffer+1 00 00 00 00 .. 00に確定する。
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
        }
        self.lower_bound = lower_bound_new;
        // 上位8bitが決定した場合、シフトを行い、決定した桁を取り出しておく
        static TOP: u32 = 1 << 24;
        while self.range < TOP {
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
            // 先頭8bitはバッファに入れたのでシフトして演算精度をあげる
            self.lower_bound <<= 8;
            self.range <<= 8;
        }
    }
}
