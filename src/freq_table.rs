//! 確率モデルの例として頻度表
use crate::alphabet_param::AlphabetParam;
use crate::decoder::Decoder;
use crate::pmodel::PModel;
use std::u32;
const MAX_ALPHABET_COUNT: usize = 10000;
/// 頻度表構造体
///
/// レンジコーダはこの頻度表をもとに計算する
pub struct FreqTable {
    /// 全アルファベットの出現頻度
    total_freq: u32,
    /// アルファベットのパラメータを保持する配列
    alphabet_params: Vec<AlphabetParam>,
}
/// コンストラクタ
impl FreqTable {
    pub fn new(alphabet_count: usize) -> Self {
        if alphabet_count > MAX_ALPHABET_COUNT {
            panic!("TooManyAlphabets!!");
        }
        FreqTable {
            total_freq: 0,
            // スタックに確保するとオーバーフローすることがあるので
            // vecでヒープにおく
            alphabet_params: vec![AlphabetParam::new(); alphabet_count],
        }
    }
}
impl PModel for FreqTable {
    fn c_freq(&self, index: usize) -> u32 {
        self.alphabet_params.get(index).unwrap().c()
    }
    fn cum_freq(&self, index: usize) -> u32 {
        self.alphabet_params.get(index).unwrap().cum()
    }
    fn total_freq(&self) -> u32 {
        self.total_freq
    }
    fn find_index(&self, decoder: &Decoder) -> usize {
        let mut left = 0;
        let mut right = self.alphabet_count() - 1;
        let rfreq = (decoder.data() - decoder.range_coder().lower_bound())
            / decoder.range_coder().range_par_total(self.total_freq());
        while left < right {
            let mid = (left + right) / 2;
            let mid_cum = self.cum_freq(mid + 1);
            if mid_cum as u64 <= rfreq {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}
/// ゲッター
impl FreqTable {
    /// アルファベットリストを取得
    pub fn alphabet_params(&self) -> &[AlphabetParam] {
        &self.alphabet_params
    }
    /// アルファベットの種類数を取得
    pub fn alphabet_count(&self) -> usize {
        self.alphabet_params.len()
    }
}
/// ロジック
impl FreqTable {
    /// アルファベットを追加
    ///
    /// アルファベットの追加が終わったら finalize()を呼ぶこと
    pub fn add_alphabet(&mut self, alphabet_index: usize) {
        self.alphabet_params[alphabet_index].add();
    }
    /// アルファベットの登録を終了
    pub fn finalize(&mut self) {
        let mut cum_total = 0;
        for i in &mut self.alphabet_params {
            i.set_cum(cum_total);
            cum_total += i.c();
        }
        self.total_freq = cum_total;
    }
}
