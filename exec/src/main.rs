use range_coder::range_coder_struct::RangeCoder;
use range_coder::simbol_data::Simbols;
use range_coder::simbol_trait::ForRangeCoder;
use std::path::Path;
fn main() {
    test_write();
    test_read();
}

fn test_write() {
    // テストデータを定義
    let data: Vec<u32> = vec![1, 2, 1, 2, 3, 3, 4, 1];
    let src: Vec<Data> = data.iter().map(|x| Data(*x)).collect();
    // シンボルデータを作る
    let mut sd: Simbols<Data> = Simbols::new(4);
    for &i in &data {
        sd.add_simbol(Data(i));
    }
    sd.finalize();
    // レンジコーダ
    let mut rc = RangeCoder::new(sd);
    for i in &src {
        println!("1シンボルエンコード開始");
        rc.encode(*i);
        println!("エンコードしたシンボル{:?}", *i);
        rc.pr();
        println!();
    }
    rc.finish();
    rc.pr();
    rc.pr_sb();
    println!("ファイル保存 at data/out.rc");
    rc.write(Path::new("data/out.rc")).unwrap();
}
fn test_read() {
    println!("ファイル読み込み");
    let mut rc = RangeCoder::<Data>::read(Path::new("data/out.rc")).unwrap();
    rc.pr_sb();
    let mut shift_count = 0;
    for _ in 0..rc.simbol_total() {
        println!("1シンボルデコード開始");
        let dec = rc.decode(&mut shift_count);
        println!("デコードされたシンボル:{:?}", dec);
        rc.pr();
        println!();
    }
}
// 以下、testで元データとするu32を使用可能にするための準備

#[derive(std::hash::Hash, Eq, PartialEq, Debug, Copy, Clone, Ord, PartialOrd)]
struct Data(u32);
impl ForRangeCoder for Data {
    fn size() -> u8 {
        4
    }
    fn save(&self) -> Vec<u8> {
        self.0.to_vec_u8()
    }
    fn read(from: &[u8]) -> Self {
        Data(UEXT::from_vec_u8(from))
    }
}
pub trait UEXT {
    fn to_vec_u8(self) -> Vec<u8>;
    fn from_vec_u8(v: &[u8]) -> Self;
}
impl UEXT for u32 {
    fn to_vec_u8(self) -> Vec<u8> {
        let mut i = self.clone();
        let mut v = Vec::new();
        v.push((i >> 24) as u8);
        i <<= 8;
        v.push((i >> 24) as u8);
        i <<= 8;
        v.push((i >> 24) as u8);
        i <<= 8;
        v.push((i >> 24) as u8);
        v
    }
    fn from_vec_u8(v: &[u8]) -> Self {
        let mut a: u32 = 0x00;
        a |= *(v.get(3).unwrap()) as u32;
        a |= (*(v.get(2).unwrap()) as u32) << 8;
        a |= (*(v.get(1).unwrap()) as u32) << 16;
        a |= (*(v.get(0).unwrap()) as u32) << 24;
        a
    }
}
