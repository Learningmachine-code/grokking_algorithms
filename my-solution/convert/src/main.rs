mod exercise;
// run test:
// cargo test -- --nocapture
fn main() {
    let s: Vec<i8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let item: i8 = 8;
    exercise::q1::binary_search(s, item);
}
