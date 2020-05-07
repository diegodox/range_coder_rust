/// レンジコーダの元データに使用できることを示すトレイト
///
/// レンジコーダに保存した際のシンボルのサイズ[byte]を返す`size`関数
/// レンジコーダに添付するデータにintoする`save`関数と、
/// レンジコーダに添付したデータからシンボルを復元する`read`関数
///
/// How to implement:
/// ```
/// fn size() -> u8
/// ```
/// ```
/// fn save(&self) -> Vec<u8>
/// ```
/// ```
/// fn read(from:&[u8]) -> Self
/// ```
pub trait ForRangeCoder {
    fn size() -> u8;
    fn save(&self) -> Vec<u8>;
    fn read(from: &[u8]) -> Self;
}
