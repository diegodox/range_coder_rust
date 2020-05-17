//! 頻度表
use crate::alphabet_param::AlphabetParam;
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
/// ゲッター
impl FreqTable {
    /// 全アルファベットの頻度合計値
    pub fn total_freq(&self) -> u32 {
        self.total_freq
    }
    /// アルファベットリストを取得
    pub fn alphabet_params(&self) -> &[AlphabetParam] {
        &self.alphabet_params
    }
    /// アルファベットのパラメータ(cとcum)を取得(imutable)
    pub fn alphabet_param(&self, alphabet_index: usize) -> &AlphabetParam {
        self.alphabet_params.get(alphabet_index).unwrap()
    }
    /// アルファベットのパラメータを取得(mutable)
    pub(crate) fn alphabet_param_mut(&mut self, alphabet_index: usize) -> &mut AlphabetParam {
        self.alphabet_params.get_mut(alphabet_index).unwrap()
    }
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
