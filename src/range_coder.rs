//! レンジコーダ(基本ロジック)
use crate::error::RangeCoderError;
use std::collections::VecDeque;
use std::u64;

const TOP8: u64 = 1 << (64 - 8);
const TOP16: u64 = 1 << (64 - 16);

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

    pub fn lower_bound(&self) -> u64 {
        self.lower_bound
    }
    pub fn range(&self) -> u64 {
        self.range
    }

    /// 1出現頻度あたりのレンジを計算
    pub fn range_par_total(&self, total_freq: u32) -> u64 {
        self.range / (total_freq as u64)
    }
    #[inline]
    pub(crate) fn set_lower_bound(&mut self, new_lower_bound: u64) {
        self.lower_bound = new_lower_bound;
    }
    #[inline]
    pub(crate) fn set_range(&mut self, range_new: u64) {
        self.range = range_new;
    }

    /// レンジ、下限をアルファベットをエンコードしたときのものにする
    ///
    /// 返値は確定させた符号
    pub(crate) fn param_update(
        &mut self,
        c_freq: u32,
        cum_freq: u32,
        total_freq: u32,
    ) -> Result<VecDeque<u8>, RangeCoderError> {
        let mut out_bytes = VecDeque::new();
        let range_par_total = self.range_par_total(total_freq);

        // update range
        self.range = range_par_total * c_freq as u64;

        // update lower-bound
        self.lower_bound = match self
            .lower_bound
            .overflowing_add(range_par_total * (cum_freq as u64))
        {
            (new_lower_bound, false) => new_lower_bound,
            // overflow means error
            (_, true) => {
                return Result::Err(RangeCoderError::LowerBoundOverflow {
                    lower_bound: self.lower_bound,
                    add_val: range_par_total * (cum_freq as u64),
                    range: self.range,
                });
            }
        };

        // 通常の桁確定
        //
        // 上位8bitは変動しない -> 左シフトで拡大してよい
        while self.lower_bound ^ self.upper_bound().unwrap() < TOP8 {
            out_bytes.push_back(self.no_carry_expansion());
        }

        // 桁上がり防止の桁確定
        //
        // レンジが不足することを防ぐために，一定よりレンジが小さくなったら
        // 上位16ビットは確定したとみなして，左シフトで拡大する
        while self.range < TOP16 {
            out_bytes.push_back(self.range_reduction_expansion());
        }

        Ok(out_bytes)
    }

    /// 下限の上位8bitを返して、レンジ、下限を8bit左シフトする
    pub(crate) fn left_shift(&mut self) -> u8 {
        let tmp = (self.lower_bound >> (64 - 8)) as u8;
        self.set_range(self.range << 8);
        self.set_lower_bound(self.lower_bound << 8);
        tmp
    }

    /// 桁確定1
    ///
    /// オーバーフローしない時の、桁確定
    /// 上位8bitを返す
    ///
    /// この関数では判定はせず、動作のみ
    /// 条件は`lower_bound^upper_bound < 1<<(64-8)`
    fn no_carry_expansion(&mut self) -> u8 {
        self.left_shift()
    }

    /// 桁確定2
    ///
    /// レンジが小さくなった時の、桁確定
    /// 上位8bitを返す
    ///
    /// この関数では判定はせず、動作のみ
    /// 条件は`range < 1<<(64-16)`
    fn range_reduction_expansion(&mut self) -> u8 {
        let range_new = !self.lower_bound & ((1 << (64 - 16)) - 1);
        self.set_range(range_new);
        self.left_shift()
    }

    /// calc upper-bound
    pub fn upper_bound(&self) -> Result<u64, RangeCoderError> {
        match self.lower_bound.overflowing_add(self.range) {
            (upper_bound, false) => Ok(upper_bound),
            (_, true) => Err(RangeCoderError::UpperBoundOverflow {
                lower_bound: self.lower_bound,
                range: self.range,
            }),
        }
    }
}
