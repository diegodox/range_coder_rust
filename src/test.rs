#[cfg(test)]
mod tests {
    use crate::decoder::Decoder;
    use crate::encoder::Encoder;
    use crate::range_coder_struct::RangeCoder;
    use crate::simbol_data::Simbols;
    #[test]
    fn test_write() {
        let test_data = vec![0, 1, 2, 3, 4, 4, 3, 2, 1, 0];
        let path_out = std::path::Path::new("test_data/test.rc");
        let mut sd = Simbols::new();
        for &i in &test_data {
            sd.add_simbol(i);
        }
        sd.finalize();
        let mut encoder = Encoder::new(RangeCoder::new(sd));
        for &i in &test_data {
            encoder.encode(i);
        }
        encoder.finish();
        encoder.write(path_out).unwrap();
    }
    #[test]
    fn test_read() {
        let test_data = vec![0, 1, 2, 3, 4, 4, 3, 2, 1, 0];
        let path_in = std::path::Path::new("test_data/test.rc");
        let decoder = Decoder::read(path_in).unwrap();
        let decoded = decoder.decode();
        println!("{:?}", decoded);
        assert_eq!(test_data, decoded);
    }
}
