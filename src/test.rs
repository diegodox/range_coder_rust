#[cfg(test)]
mod tests {
    use crate::decoder::Decoder;
    use crate::encoder::Encoder;
    use crate::range_coder_struct::RangeCoder;
    use crate::simbol_data::Simbols;
    #[test]
    fn test_encode_and_decode() {
        let test_data = vec![2, 1, 1, 3, 1, 4, 2, 1, 0, 1, 5, 9, 8, 7, 6, 5];
        let mut sd = Simbols::new();
        for &i in &test_data {
            sd.add_simbol(i);
        }
        sd.finalize();
        for i in sd.simbol_paramaters() {
            if i.c() == 0 {
                break;
            }
            println!("c:{},cum:{}", i.c(), i.cum());
        }
        println!("encode");
        let mut encoder = Encoder::new(RangeCoder::new());
        for &i in &test_data {
            println!("encode {}", i);
            encoder.encode(sd.simbol_param(i), sd.total_freq());
        }
        encoder.finish();
        print!("output : 0x");
        for i in encoder.data() {
            print!("{:x}", i);
        }
        print!("\n\n");

        let mut decoder = Decoder::new(RangeCoder::new().into_encoder());
        decoder.set_data(encoder.data().to_owned());
        decoder.decode_start();
        for _ in 0..test_data.len() {
            decoder.decode_one_simbol(&sd);
        }
    }
}
