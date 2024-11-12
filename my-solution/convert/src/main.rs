mod exercise;
// run test:
// cargo test -- --nocapture
fn main() {
    let a = &[1, 2, 3, 4, 5, 34, 12];
    let b = exercise::q4_3::max(a, a[0]);
    println!("{b}");
}
