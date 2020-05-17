//! アルファベットの頻度や累積頻度を保持する構造体
#[derive(Clone, Copy, Debug)]
/// アルファベットの出現回数を示す構造体
pub struct AlphabetParam {
    /// 文字の累積出現頻度
    pub(crate) cum: u32,
    /// 文字の出現頻度
    pub(crate) c: u32,
}
/// コンストラクタ
impl AlphabetParam {
    pub fn new() -> Self {
        AlphabetParam { cum: 0, c: 0 }
    }
}
/// ゲッター
impl AlphabetParam {
    pub fn c(&self) -> u32 {
        self.c
    }
    pub fn cum(&self) -> u32 {
        self.cum
    }
}
/// セッター
impl AlphabetParam {
    pub fn set_c(&mut self, c: u32) {
        self.c = c;
    }
    pub fn set_cum(&mut self, cum: u32) {
        self.cum = cum;
    }
}
/// other
impl AlphabetParam {
    /// 出現頻度を1回ふやす
    pub(crate) fn add(&mut self) {
        self.c += 1;
    }
}
