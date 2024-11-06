mod exercise;
// run test:
// cargo test -- --nocapture
fn main() {
    let x = 6;
    let y = exercise::q3::fact(x);
    println!("{y}");
}
