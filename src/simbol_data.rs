use crate::simbol_trait::ForRangeCoder;
use crate::uext::UEXT;
use std::u32;

pub const MAX_SIMBOL_COUNT: usize = 256;
#[derive(Clone, Copy, Debug)]
/// シンボルの出現回数を示す構造体
pub struct SimbolParam {
    /// 文字の累積出現回数
    pub(crate) cum: u32,
    /// 文字の出現回数
    pub(crate) c: u32,
}
/// SimbolParamをRangeCoderで読み書きできるようにする関数
impl ForRangeCoder for SimbolParam {
    fn size() -> u8 {
        4
    }
    /// 書き込み
    fn save(&self) -> Vec<u8> {
        let mut tmp = Vec::new();
        tmp.append(&mut self.c.to_vec_u8());
        tmp
    }
    /// 読み込み
    fn read(v: &[u8]) -> Self {
        let c = UEXT::from_vec_u8(v);
        SimbolParam { cum: 0, c: c }
    }
}
// コンストラクタをimpl
impl SimbolParam {
    fn new() -> Self {
        SimbolParam { cum: 0, c: 0 }
    }
}
// ゲッターをimpl
impl SimbolParam {
    pub(crate) fn c(&self) -> u32 {
        self.c
    }
    pub(crate) fn cum(&self) -> u32 {
        self.cum
    }
}
// セッターをimpl
impl SimbolParam {
    pub(crate) fn set_c(&mut self, c: u32) {
        self.c = c;
    }
    pub(crate) fn set_cum(&mut self, cum: u32) {
        self.cum = cum;
    }
}
// 他の関数をimpl
impl SimbolParam {
    /// 出現回数を1回ふやす
    fn add(&mut self) {
        self.c += 1;
    }
}

/// シンボル関連のデータを管理する構造体
///
/// まず、この構造体にシンボルを用意する
pub struct Simbols {
    /// 全文字の出現回数
    total_freq: u32,
    /// シンボルのパラメータを保持する配列
    simbol_paramaters: [SimbolParam; MAX_SIMBOL_COUNT],
}
/// コンストラクタをimpl
impl Simbols {
    pub fn new() -> Self {
        Simbols {
            total_freq: 0,
            simbol_paramaters: [SimbolParam::new(); MAX_SIMBOL_COUNT],
        }
    }
}
// ゲッターをimpl
impl Simbols {
    pub(crate) fn total_freq(&self) -> u32 {
        self.total_freq
    }
    pub(crate) fn simbol_paramaters(&self) -> &[SimbolParam] {
        &self.simbol_paramaters
    }
    /// シンボルのパラメータ(cとcum)を取得(imutable)
    pub fn simbol_param(&self, simbol_index: usize) -> &SimbolParam {
        self.simbol_paramaters.get(simbol_index).unwrap()
    }
    /// シンボルのパラメータを取得(mutable)
    pub(crate) fn simbol_param_mut(&mut self, simbol_index: usize) -> &mut SimbolParam {
        self.simbol_paramaters.get_mut(simbol_index).unwrap()
    }
}
// 他の関数をimpl
impl Simbols {
    /// シンボルを追加
    ///
    /// シンボルの追加が終わったら finalize()を呼ぶこと
    pub fn add_simbol(&mut self, simbol_index: usize) {
        self.simbol_param_mut(simbol_index).add();
    }
    /// シンボルの登録を終了
    pub fn finalize(&mut self) {
        let mut cum_total = 0;
        for i in 0..MAX_SIMBOL_COUNT {
            self.simbol_param_mut(i).set_cum(cum_total);
            cum_total += self.simbol_param(i).c();
        }
        self.total_freq = cum_total;
    }
}
