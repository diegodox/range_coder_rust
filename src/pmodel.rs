//! レンジコーダの確率モデルに必要なメソッドを定義したトレイト
use crate::decoder::Decoder;

pub trait PModel {
    /// インデックスから頻度を返す
    fn c_freq(&self, index: usize) -> u32;
    /// インデックスから累積頻度を返す
    fn cum_freq(&self, index: usize) -> u32;
    /// 合計累積頻度を返す
    fn total_freq(&self) -> u32;
    /// PModelとデコーダを使って，デコードするインデックスを見つける
    fn find_index(&self, decoder: &Decoder) -> usize;
    /// 理想符号長を返す
    fn ideal_code_length(&self, index: usize) -> Result<f64, String> {
        let p_collect = self.c_freq(index) as f64;
        if p_collect == 0f64 {
            return Err(r#"code length is undefind when probability is zero"#.to_string());
        }
        if p_collect.is_nan() | p_collect.is_infinite() {
            return Err(format!(
                "code length is undefind when probability is nan or infinite as {:?}",
                p_collect
            ));
        }
        if p_collect.is_sign_negative() {
            return Err(format!(
                "code length is undefind when probability is negative as {}",
                p_collect
            ));
        }
        let p_sum = self.total_freq() as f64;
        let code_length = (p_sum.ln() - p_collect.ln()) / std::f64::consts::LN_2;
        debug_assert!(
            code_length.is_finite(),
            "p_sum: {}, p_collect: {}",
            p_sum,
            p_collect,
        );
        Ok(code_length)
    }
}
