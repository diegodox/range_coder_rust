use range_coder;
fn main() {
    test();
    println!("hello")
}
fn test() {
    let data: Vec<u32> = vec![1, 2, 1, 2, 3, 3, 4, 1];
    let mut rc = range_coder::RangeCoder::new();
    println!("start encoding.");
    for &i in &data {
        rc.add_simbol(i);
    }
    rc.reflesh_cum();
    println!("finish c,cum");
    for i in &data {
        println!("\nlet encode '{}'", i);
        rc.encode(*i);
    }
    rc.finish();
    rc.pr();
}
