use crate::decoder;
use crate::encoder;
use crate::simbol_data::SimbolParam;
use crate::simbol_data::Simbols;
use std::u32;

/// **RangeCoder構造体**
///
/// RangeCoder<シンボルのデータ型>で指定
pub struct RangeCoder {
    /// 下限
    lower_bound: u32,
    /// 幅
    range: u32,
    /// シンボルのデータ
    simbol_data: Simbols,
}
impl RangeCoder {
    /// コンストラクタ的なやつ
    ///
    /// 先に作成したシンボルデータを引数にとる
    pub fn new(simbol_data_src: Simbols) -> Self {
        RangeCoder {
            lower_bound: 0,
            range: u32::MAX,
            simbol_data: simbol_data_src,
        }
    }
    /// エンコーダを作成
    pub fn into_encoder(self) -> encoder::Encoder {
        encoder::Encoder::new(self)
    }
    /// デコーダを作成
    pub fn into_decoder(self) -> decoder::Decoder {
        decoder::Decoder::new(self)
    }
    /// シンボルをエンコードしたときの、レンジを取得
    pub(crate) fn range_when_encode(&self, simbol_param: &SimbolParam) -> u32 {
        let range_before = self.range() / self.simbol_total();
        match (simbol_param.cum() + simbol_param.c()).cmp(&self.simbol_data.total()) {
            // レンジ最後のシンボルの場合、通常のレンジ更新で発生する誤差(整数除算によるもの)を含める
            std::cmp::Ordering::Equal => {
                return self.range() - (range_before * simbol_param.cum());
            }
            // レンジ最後のシンボルでない場合、通常のレンジ更新を行う
            std::cmp::Ordering::Less => {
                return range_before * simbol_param.c();
            }
            // Graterになることはない
            std::cmp::Ordering::Greater => panic!(),
        }
    }
    /// シンボルをエンコードしたときの、下限と確定した桁があるかどうかを取得
    pub(crate) fn lower_bound_when_encode(&self, simbol_param: &SimbolParam) -> (u32, bool) {
        let range_before = self.range() / self.simbol_total();
        self.lower_bound()
            .overflowing_add(range_before * simbol_param.cum())
    }
}
/// ゲッタ
impl RangeCoder {
    /// シンボルデータのゲッタ
    pub fn simbol_data(&self) -> &Simbols {
        &self.simbol_data
    }
    /// シンボルの合計出現回数のゲッタ
    pub(crate) fn simbol_total(&self) -> u32 {
        self.simbol_data.total()
    }
    /// レンジのゲッタ
    pub fn range(&self) -> u32 {
        self.range
    }
    /// 下限のゲッタ
    pub fn lower_bound(&self) -> u32 {
        self.lower_bound
    }
}
/// セッタ
impl RangeCoder {
    /// シンボルデータのセッタ
    pub fn swap_simbol_data(&mut self, simbol_data: Simbols) {
        self.simbol_data = simbol_data;
    }
    /// 下限のセッタ
    pub(crate) fn set_lower_bound(&mut self, lower_bound_new: u32) {
        self.lower_bound = lower_bound_new;
    }
    /// レンジのセッタ
    pub(crate) fn set_range(&mut self, range_new: u32) {
        self.range = range_new;
    }
}
