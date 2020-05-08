//! This module is range_coder.
//!
//! 参考文献:[CodeZine: 高速な算術符号を実現する「Range Coder」](https://codezine.jp/article/detail/443)
//!
//! # 使用方法
//!
//! ## エンコード、デコード共通する準備
//! ```
//! // レンジコーダのシンボルに使うデータ型(ここではSimbol)にForRangeCoderトレイトをimplする
//! impl ForRangeCoder for Simbol {
//!     fn size() -> u8 {
//!         // シンボル保存時のサイズを返す
//!     }
//!     fn save(&self) -> Vec<u8> {
//!         // シンボルをVec<u8>型に変換
//!     }
//!     fn read(from: &[u8]) -> Self {
//!         //u8のスライスをシンボルに変換
//!     }
//! }
//! ```
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
//! // シンボルデータからレンジコーダ構造体を作る
//! let mut rangecoder = RangeCoder::new(simbol_data);
//!
//! // 1シンボルずつエンコードしていく
//! rangecoder.encode(simbol);
//!
//! // エンコードし終わったらfinishを呼ぶ
//! rangecoder.finish();
//!
//! // 出力をファイルに保存するには
//! rc.write(Path::new("保存場所"));
//! ```
//!
//! ## デコード
//! ```
//! // ファイルからレンジコーダ構造体を復元する
//! let mut rangecoder = RangeCoder::<Simbol>::read(Path::new("ファイルの場所")).unwrap();
//!
//! // 全文字デコードする
//! let decoded = rangecoder.decode();
//!
//! // popで先にエンコードしたものから順に取り出せる
//! decoded.pop()
//! ```
pub mod range_coder_struct;
pub mod simbol_data;
pub mod simbol_trait;
pub use range_coder_struct::decoder;
pub use range_coder_struct::encoder;
pub(crate) mod uext;
