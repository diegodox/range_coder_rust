use range_coder::{decoder::Decoder, encoder::Encoder, pmodel::PModel};

#[derive(Clone, Copy, Debug)]
/// アルファベットの出現回数を示す構造体
struct AlphabetParam {
    /// 文字の累積出現頻度
    cum: u32,
    /// 文字の出現頻度
    c: u32,
}
struct FreqTable {
    /// 全アルファベットの出現頻度
    total_freq: u32,
    /// アルファベットのパラメータを保持する配列
    alphabet_params: Vec<AlphabetParam>,
}

impl PModel for FreqTable {
    fn c_freq(&self, index: usize) -> u32 {
        self.alphabet_params.get(index).unwrap().c
    }
    fn cum_freq(&self, index: usize) -> u32 {
        self.alphabet_params.get(index).unwrap().cum
    }
    fn total_freq(&self) -> u32 {
        self.total_freq
    }
    fn find_index(&self, decoder: &Decoder) -> usize {
        // 符号に対応するcum
        let rfreq = (decoder.data() - decoder.range_coder().lower_bound())
            / decoder.range_coder().range_par_total(self.total_freq());

        // 2分探索で `rfreq` を探す
        let mut left = 0;
        let mut right = self.alphabet_count() - 1;
        while left < right {
            let mid = (left + right) / 2;
            let mid_cum = self.cum_freq(mid + 1);
            if mid_cum as u64 <= rfreq {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

impl FreqTable {
    fn new(alphabet_count: usize) -> Self {
        FreqTable {
            total_freq: 0,
            alphabet_params: vec![AlphabetParam { cum: 0, c: 0 }; alphabet_count],
        }
    }
    fn alphabet_count(&self) -> usize {
        self.alphabet_params.len()
    }
    fn add_alphabet_freq(&mut self, alphabet_index: usize) {
        self.alphabet_params[alphabet_index].c += 1;
    }
    fn calc_cum(&mut self) {
        self.total_freq = self
            .alphabet_params
            .iter_mut()
            .fold(0, |cum_total, alphabet| {
                alphabet.cum = cum_total;
                cum_total + alphabet.c
            })
    }
}

fn main() {
    // define test data
    let test_data = vec![2, 1, 1, 4, 1, 4, 2, 1, 0, 1, 5, 9, 8, 7, 6, 5];

    // create freq-table
    let mut sd = FreqTable::new(10);
    for &i in &test_data {
        sd.add_alphabet_freq(i);
    }
    sd.calc_cum();
    {
        println!("FREQ TABLE");
        for i in 0..sd.alphabet_params.len() {
            println!("index:{}, c:{}, cum:{}", i, sd.c_freq(i), sd.cum_freq(i));
        }
        println!();
    }

    // encode
    println!("ENCODING");
    let mut encoder = Encoder::new();
    print!("encode : ");
    for &i in &test_data {
        print!("{},", i);
        encoder.encode::<FreqTable>(&sd, i);
    }
    let code = encoder.finish();
    {
        println!();
        print!("output : 0x");
        for i in &code {
            print!("{:x}", i);
        }
        print!("\nlength : {}byte", code.len());
        print!("\n\n");
    }

    // decode
    let mut decoder = Decoder::new(code);
    println!("DECODING");
    print!("decode : ");
    let decodeds = test_data
        .iter()
        .map(|_| {
            let decoded = decoder.decode(&sd);
            print!("{},", decoded);
            decoded
        })
        .collect::<Vec<_>>();

    // test
    assert_eq!(decodeds, test_data);

    println!();
    println!();
    println!("test passed🎉");
}
