//! This module is range_coder.
//!
//! 参考文献:[CodeZine: 高速な算術符号を実現する「Range Coder」](https://codezine.jp/article/detail/443)
pub mod range_coder_struct;
pub mod simbol_data;
pub mod simbol_trait;
pub use range_coder_struct::decoder;
pub use range_coder_struct::encoder;
pub(crate) mod uext;
