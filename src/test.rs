#[cfg(test)]
mod tests {
    use crate::decoder::Decoder;
    use crate::encoder::Encoder;
    use crate::simbol_data::Simbols;
    #[test]
    fn test_encode_and_decode() {
        // テストデータを定義
        let test_data = vec![2, 1, 1, 3, 1, 4, 2, 1, 0, 1, 5, 9, 8, 7, 6, 5];
        // シンボルデータを準備
        let mut sd = Simbols::new();
        for &i in &test_data {
            // シンボルを追加していく
            sd.add_simbol(i);
        }
        // シンボルの追加を終了
        sd.finalize();
        // （確認用)シンボルデータのプリント
        for i in sd.simbol_paramaters() {
            if i.c() == 0 {
                break;
            }
            println!("c:{},cum:{}", i.c(), i.cum());
        }
        println!("encode");
        // エンコーダを準備
        let mut encoder = Encoder::new();
        // 1シンボルずつエンコード
        for &i in &test_data {
            println!("encode {}", i);
            encoder.encode(sd.simbol_param(i), sd.total_freq());
        }
        // エンコード終了処理
        encoder.finish();
        // (確認用)エンコード出力のプリント
        print!("output : 0x");
        for i in encoder.data() {
            print!("{:x}", i);
        }
        print!("\n\n");

        // デコーダを準備
        let mut decoder = Decoder::new();
        // エンコーダの出力をデコーダにセット
        decoder.set_data(encoder.data().to_owned());
        // デコード開始処理
        decoder.decode_start();
        // 1文字ずつデコード
        println!("デコードされたインデックスは");
        for _ in 0..test_data.len() {
            print!("{} , ", decoder.decode_one_simbol(&sd));
        }
    }
}
