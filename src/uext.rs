pub(crate) trait UEXT {
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
