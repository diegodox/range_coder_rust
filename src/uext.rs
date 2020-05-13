pub(crate) trait UEXT {
    fn to_vec_u8(self) -> Vec<u8>;
    fn from_vec_u8(v: &[u8]) -> Self;
}
impl UEXT for u32 {
    fn to_vec_u8(self) -> Vec<u8> {
        let mut i = self.clone();
        let mut v = Vec::new();
        for _ in 0..std::mem::size_of::<Self>() {
            v.push((i >> (std::mem::size_of::<Self>() - 1) * 8) as u8);
            i <<= 8;
        }
        v
    }
    fn from_vec_u8(v: &[u8]) -> Self {
        let mut a: Self = 0;
        for j in 0..v.len() {
            a |= (*(v.get(j as usize).unwrap()) as Self) << 8 * (v.len() - 1 - j);
        }
        a
    }
}

impl UEXT for usize {
    fn to_vec_u8(self) -> Vec<u8> {
        let mut i = self.clone();
        let mut v = Vec::new();
        for _ in 0..std::mem::size_of::<Self>() {
            v.push((i >> (std::mem::size_of::<Self>() - 1) * 8) as u8);
            i <<= 8;
        }
        v
    }
    fn from_vec_u8(v: &[u8]) -> Self {
        let mut a: Self = 0;
        for j in 0..v.len() {
            a |= (*(v.get(j as usize).unwrap()) as Self) << 8 * (v.len() - 1 - j);
        }
        a
    }
}

impl UEXT for u64 {
    fn to_vec_u8(self) -> Vec<u8> {
        let mut i = self.clone();
        let mut v = Vec::new();
        for _ in 0..std::mem::size_of::<Self>() {
            v.push((i >> (std::mem::size_of::<Self>() - 1) * 8) as u8);
            i <<= 8;
        }
        v
    }
    fn from_vec_u8(v: &[u8]) -> Self {
        let mut a: Self = 0;
        for j in 0..v.len() {
            a |= (*(v.get(j as usize).unwrap()) as Self) << 8 * (v.len() - 1 - j);
        }
        a
    }
}

#[test]
fn testuext() {
    let a: usize = 0x010fa5;
    println!("{:x}", a);
    let v = a.to_vec_u8();
    for i in &v {
        println!("{:x}", i);
    }
    let b: usize = UEXT::from_vec_u8(&v);
    assert_eq!(a, b);
}
