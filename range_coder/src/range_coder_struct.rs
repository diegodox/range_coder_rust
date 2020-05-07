use crate::simbol_data::Simbols;
use crate::simbol_trait::ForRangeCoder;
use std::collections::VecDeque;
use std::u32;

pub mod decoder;
pub mod encoder;

/// **RangeCoder構造体**
///
/// RangeCoder<シンボルのデータ型>で指定
pub struct RangeCoder<T>
where
    T: Eq + std::hash::Hash + ForRangeCoder + Ord,
{
    /// 符号
    pub(crate) data: VecDeque<u8>,
    /// 下限
    pub(crate) lower_bound: u32,
    /// 幅
    pub(crate) range: u32,
    /// シンボルのデータ
    pub(crate) simbol_data: Simbols<T>,
    /// 未確定桁を格納するバッファ
    pub(crate) buffer: Option<u8>,
    /// 0xff or 0x00 になる値の個数
    /// (参考文献でcarryNと呼ばれるもの)
    pub(crate) carry_n: u32,
}
impl<T> RangeCoder<T>
where
    T: Eq + std::hash::Hash + ForRangeCoder + Ord + std::fmt::Debug,
{
    /// デバッグ用出力
    pub fn pr(&self) {
        println!("   ENCODER STATE");
        print!("      data        :");
        for i in &(self.data) {
            print!("0x{:x} , ", i);
        }
        println!("");
        match self.buffer {
            Some(b) => println!("      buffer      :0x{:x}", b),
            None => println!("      buffer      :None"),
        }
        println!("      carry_n     :{}個", self.carry_n);
        println!("      lower_bound :0x{:x}", &(self.lower_bound));
        println!("      range       :0x{:x}", &(self.range));
    }
    pub fn pr_sb(&self) {
        println!("simbol data is:");
        println!("{:?}", self.simbol_data);
    }
    /// コンストラクタ的なやつ
    ///
    /// 先に作成したシンボルデータを引数にとる
    pub fn new(simbol_data_src: Simbols<T>) -> Self {
        RangeCoder {
            data: VecDeque::new(),
            lower_bound: 0,
            range: u32::MAX,
            simbol_data: simbol_data_src,
            buffer: None,
            carry_n: 0,
        }
    }
    pub fn count_simbol_type(&self) -> u32 {
        self.simbol_data.simbol_type_count
    }
    /// シンボルの合計出現回数を返す
    pub fn simbol_total(&self) -> u32 {
        self.simbol_data.total
    }
}
