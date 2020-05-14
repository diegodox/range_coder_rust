//! This module is range_coder.
//!
//! 参考文献:[CodeZine: 高速な算術符号を実現する「Range Coder」](https://codezine.jp/article/detail/443)
//!
//! # 使用方法
//!
//! ## エンコード
//!
//! ```
//! // Simbols構造体を用意する
//! let mut simbol_data: Simbols<Data> = Simbols::new(Simbol::size());
//!
//! // 次に、シンボルを登録していく
//! // 1シンボル追加する方法
//! simbol_data.add_simbol(simbol);
//!
//! // シンボルを登録し終わったらfinalizeする
//! simbol_data.finalize();
//!
//! // エンコーダ構造体を作る
//! let mut encoder = Encoder::new();
//!
//! // 1シンボルずつエンコードしていく
//! // ここではインデックスiをエンコードする
//! encoder.encode(simbol_data.simbol_param(i), simbol_data.total_freq());
//!
//! // エンコードし終わったらfinishを呼ぶ
//! encoder.finish();
//!
//! // エンコーダの出力は次のように取り出せる
//! let output = encoder.data();
//! ```
//!
//! ## デコード
//! ```
//! // デコーダ構造体を作る
//! let mut dc = Decoder::new();
//!
//! // デコーダに復元したいエンコーダ出力をセットする
//! dc.set_data(data);
//!
//! // デコード開始前に必ず呼び出す
//! dc.decode_start();
//!
//! // 1シンボル、デコードする(シンボルのインデックスを返す)
//! let decoded = dc.decode_one_simbol(simbols);
//! ```
pub mod decoder;
pub mod encoder;
pub mod range_coder_struct;
pub mod simbol_data;
pub mod simbol_trait;
pub(crate) mod test;
pub(crate) mod uext;
