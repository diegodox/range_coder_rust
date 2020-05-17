//! This module is range_coder.
//!
//! 参考文献:[CodeZine: 高速な算術符号を実現する「Range Coder」](https://codezine.jp/article/detail/443)
//!
//! # 使用方法
//!
//! ## 頻度表を準備する
//! ```
//! // 頻度表本体
//! let mut freq_table = FreqTable::new(10);
//! // 頻度表にアルファベットを追加していく
//! for &i in 元データ {
//!     freq_table.add_alphabet(i);
//! }
//! // アルファベットの追加を終了
//! freq_table.finalize();
//! ```
//! ## エンコード
//! ```
//! エンコーダを準備
//! let mut encoder = Encoder::new();
//! 1アルファベットずつエンコード
//! for &i in 元データ {
//!    println!("encode {}", i);
//!    encoder.encode(freq_table.alphabet_param(i), freq_table.total_freq());
//! }
//! エンコード終了処理
//! encoder.finish();
//! ```
//! ## デコード
//! ```
//! デコーダを準備
//! let mut decoder = Decoder::new();
//! エンコーダの出力をデコーダにセット
//! decoder.set_data(encoder.data().to_owned());
//! デコード開始処理
//! decoder.decode_start();
//! 1文字ずつデコード
//! for i in 0..元データの長さ {
//!     let decoded = decoder.decode_one_alphabet(&freq_table);
//! }
//! ```
pub mod alphabet_param;
pub mod decoder;
pub mod encoder;
pub mod freq_table;
pub mod range_coder;
pub(crate) mod test;
