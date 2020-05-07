use crate::simbol_trait::ForRangeCoder;
use crate::uext::UEXT;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::u32;

/// シンボル関連のデータを管理する構造体
///
/// まず、この構造体にシンボルを用意する
#[derive(Debug)]
pub struct Simbols<T>
where
    T: Eq + std::hash::Hash + ForRangeCoder,
{
    /// シンボル保存時のサイズ[byte]
    pub(crate) size: u8,
    /// 全文字の出現回数
    pub(crate) total: u32,
    /// シンボルの種類数
    pub(crate) simbol_type_count: u32,
    /// シンボルとパラメータのインデックスの対応
    pub(crate) index: BTreeMap<T, u32>,
    /// シンボルのパラメータを保持する配列
    pub(crate) simbol_paramaters: VecDeque<SimbolParam>,
}

#[derive(Debug)]
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
        let cum = UEXT::from_vec_u8(&v[0..4]);
        let c = UEXT::from_vec_u8(&v[4..8]);
        SimbolParam { cum: cum, c: c }
    }
}
impl SimbolParam {
    fn new() -> Self {
        SimbolParam { cum: 0, c: 1 }
    }
    pub fn new_with_c(c: u32) -> Self {
        SimbolParam { cum: 0, c: c }
    }
    fn add(&mut self) {
        self.c += 1;
    }
}
impl<T> Simbols<T>
where
    T: std::cmp::Eq + std::hash::Hash + ForRangeCoder + Ord,
{
    pub fn new(size: u8) -> Self {
        Simbols {
            size: size,
            total: 0,
            simbol_type_count: 0,
            index: BTreeMap::new(),
            simbol_paramaters: VecDeque::new(),
        }
    }
    /// シンボルを追加
    ///
    /// シンボルの追加が終わったら finalize()を呼ぶこと
    pub fn add_simbol(&mut self, simbol: T) {
        match self.index.entry(simbol) {
            std::collections::btree_map::Entry::Occupied(o) => {
                self.simbol_paramaters[*(o.get()) as usize].add();
            }
            std::collections::btree_map::Entry::Vacant(v) => {
                // 全シンボル数を追加
                self.simbol_type_count += 1;
                // simbol_parametersに登録
                self.simbol_paramaters.push_back(SimbolParam::new());
                // 新しいシンボルのインデックスを挿入
                v.insert(self.simbol_type_count - 1);
            }
        }
    }
    /// シンボルのパラメータ(cとcum)を取得(キーから)
    pub(crate) fn get(&self, simbol: &T) -> Option<&SimbolParam> {
        Some(
            &(self.simbol_paramaters[match self.index.get(&simbol) {
                Some(i) => *i as usize,
                None => return None,
            }]),
        )
    }
    /// シンボルのパラメータを取得(indexから)
    pub(crate) fn get_by_index(&self, index: u32) -> Option<&SimbolParam> {
        self.simbol_paramaters.get(index as usize)
    }
    /// シンボルの登録を終了
    pub fn finalize(&mut self) {
        let mut cum_total = 0;
        for i in &mut (self.simbol_paramaters) {
            i.cum = cum_total;
            cum_total += i.c;
        }
        self.total = cum_total;
    }
}
