//! レンジコーダの確率モデルに必要なメソッドを定義したトレイト
use crate::decoder::Decoder;

pub trait PModel {
    /// インデックスから頻度を返す
    fn c_freq(&self, index: usize) -> u32;
    /// インデックスから累積頻度を返す
    fn cum_freq(&self, index: usize) -> u32;
    /// 合計累積頻度を返す
    fn total_freq(&self) -> u32;
    /// PModelとレンジコーダからインデックスを返す
    fn find_index(&self, decoder: &Decoder) -> usize;
}
