use crate::alphabet_param::AlphabetParam;
use crate::decoder;
use crate::encoder;
use std::u64;

/// **RangeCoder構造体**
pub struct RangeCoder {
    /// 下限
    lower_bound: u64,
    /// 幅
    range: u64,
}
impl RangeCoder {
    /// コンストラクタ的なやつ
    pub fn new() -> Self {
        RangeCoder {
            lower_bound: 0,
            range: u64::MAX,
        }
    }
    /// エンコーダを作成
    // 説明会用:使わない
    pub fn into_encoder(self) -> encoder::Encoder {
        let mut ec = encoder::Encoder::new();
        ec.set_range_coder(self);
        ec
    }
    /// デコーダを作成
    // 説明会用:使わない
    pub fn into_decoder(self) -> decoder::Decoder {
        let mut dc = decoder::Decoder::new();
        dc.set_encoder(self.into_encoder());
        dc
    }
    pub(crate) fn range_par_total(&self, total_freq: u32) -> u64 {
        self.range() / total_freq as u64
    }
    /// レンジ、下限をアルファベットをエンコードしたときのものにする
    ///
    /// 引数
    /// alphabet_param : エンコードするアルファベットのパラメータ
    /// total_freq : 全アルファベットの合計出現回数
    pub(crate) fn param_update(&mut self, alphabet_param: &AlphabetParam, total_freq: u32) {
        let range_par_total = self.range_par_total(total_freq);
        self.set_range(range_par_total * alphabet_param.c() as u64);
        self.set_lower_bound(self.lower_bound() + (range_par_total * (alphabet_param.cum() as u64)))
    }
    /// 下限の上位8bitを返して、レンジ、下限を8bit左シフトする
    pub(crate) fn left_shift(&mut self) -> u8 {
        let tmp = (self.lower_bound >> (64 - 8)) as u8;
        self.set_range(self.range() << 8);
        self.set_lower_bound(self.lower_bound << 8);
        tmp
    }
}
/// ゲッタ
impl RangeCoder {
    /// レンジのゲッタ
    pub fn range(&self) -> u64 {
        self.range
    }
    /// 下限のゲッタ
    pub fn lower_bound(&self) -> u64 {
        self.lower_bound
    }
}
/// セッタ
impl RangeCoder {
    /// 下限のセッタ
    pub(crate) fn set_lower_bound(&mut self, lower_bound_new: u64) {
        self.lower_bound = lower_bound_new;
    }
    /// レンジのセッタ
    pub(crate) fn set_range(&mut self, range_new: u64) {
        self.range = range_new;
    }
}
