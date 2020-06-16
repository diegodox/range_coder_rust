//! レンジコーダ(基本ロジック)
use crate::error::RangeCoderError;
use std::collections::VecDeque;
use std::u64;

/// RangeCoder構造体
pub struct RangeCoder {
    /// 下限
    lower_bound: u64,
    /// 幅
    range: u64,
}
impl Default for RangeCoder {
    fn default() -> Self {
        RangeCoder {
            lower_bound: 0,
            range: u64::MAX,
        }
    }
}
/// コンストラクタ
impl RangeCoder {
    /// コンストラクタ
    pub fn new() -> Self {
        RangeCoder::default()
    }
}
/// ロジック
impl RangeCoder {
    /// レンジ、下限をアルファベットをエンコードしたときのものにする
    ///
    /// 引数
    /// alphabet_param : エンコードするアルファベットのパラメータ
    /// total_freq : 全アルファベットの合計出現回数
    pub(crate) fn param_update(
        &mut self,
        c_freq: u32,
        cum_freq: u32,
        total_freq: u32,
    ) -> Result<VecDeque<u8>, RangeCoderError> {
        let mut out_bytes = VecDeque::new();
        let range_par_total = self.range_par_total(total_freq);
        self.set_range(range_par_total * c_freq as u64);
        let (lower_bound_new, is_overflow) = self
            .lower_bound()
            .overflowing_add(range_par_total * (cum_freq as u64));
        if is_overflow {
            return Result::Err(RangeCoderError::LowerBoundOverflow {
                lower_bound: self.lower_bound(),
                add_val: range_par_total * (cum_freq as u64),
                range: self.range,
            });
        }
        self.set_lower_bound(lower_bound_new);
        const TOP8: u64 = 1 << (64 - 8);
        const TOP16: u64 = 1 << (64 - 16);
        while self.lower_bound() ^ self.upper_bound().unwrap() < TOP8 {
            out_bytes.push_back(self.no_carry_expansion());
        }
        while self.range() < TOP16 {
            out_bytes.push_back(self.range_reduction_expansion());
        }
        Result::Ok(out_bytes)
    }
    /// 下限の上位8bitを返して、レンジ、下限を8bit左シフトする
    pub(crate) fn left_shift(&mut self) -> u8 {
        let tmp = (self.lower_bound >> (64 - 8)) as u8;
        self.set_range(self.range() << 8);
        self.set_lower_bound(self.lower_bound << 8);
        tmp
    }
    /// 桁確定1
    ///
    /// オーバーフローしない時の、桁確定
    /// この関数では判定はせず、動作のみ
    /// 条件は`lower_bound^upper_bound < 1<<(64-8)`
    fn no_carry_expansion(&mut self) -> u8 {
        self.left_shift()
    }
    /// 桁確定2
    ///
    /// レンジが小さくなった時の、桁確定
    /// この関数では判定はせず、動作のみ
    /// 条件は`range < 1<<(64-16)`
    fn range_reduction_expansion(&mut self) -> u8 {
        let range_new = !(self.lower_bound() & ((1 << (64 - 16)) - 1));
        self.set_range(range_new);
        self.left_shift()
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
    /// 上限のゲッタ
    pub fn upper_bound(&self) -> Result<u64, RangeCoderError> {
        let (v, b) = self.lower_bound().overflowing_add(self.range());
        if b {
            Err(RangeCoderError::UpperBoundOverflow {
                lower_bound: self.lower_bound(),
                range: self.range(),
            })
        } else {
            Ok(v)
        }
    }
    /// 1出現頻度あたりのレンジを計算
    pub(crate) fn range_par_total(&self, total_freq: u32) -> u64 {
        self.range() / total_freq as u64
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
