use crate::decoder;
use crate::encoder;
use crate::simbol_data::Simbols;
use std::collections::VecDeque;
use std::u32;

/// **RangeCoder構造体**
///
/// RangeCoder<シンボルのデータ型>で指定
pub struct RangeCoder {
    /// 符号
    pub(crate) data: VecDeque<u8>,
    /// 下限
    pub(crate) lower_bound: u32,
    /// 幅
    pub(crate) range: u32,
    /// シンボルのデータ
    pub(crate) simbol_data: Simbols,
}
impl RangeCoder {
    /// デバッグ用出力
    pub fn pr(&self) {
        println!("   ENCODER STATE");
        print!("      data        :");
        for i in &(self.data) {
            print!("0x{:x} , ", i);
        }
        println!("");
        println!("      lower_bound :0x{:x}", &(self.lower_bound));
        println!("      range       :0x{:x}", &(self.range));
    }
    /// コンストラクタ的なやつ
    ///
    /// 先に作成したシンボルデータを引数にとる
    pub fn new(simbol_data_src: Simbols) -> Self {
        RangeCoder {
            data: VecDeque::new(),
            lower_bound: 0,
            range: u32::MAX,
            simbol_data: simbol_data_src,
        }
    }
    /// シンボルの合計出現回数を返す
    pub(crate) fn simbol_total(&self) -> u32 {
        self.simbol_data.total
    }
    pub fn into_encoder(self) -> encoder::Encoder {
        encoder::Encoder::new(self)
    }
    pub fn into_decoder(self) -> decoder::Decoder {
        decoder::Decoder::new(self)
    }
}
