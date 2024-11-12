pub fn fact(x: u32) -> u32 {
    if x == 1 {
        return 1;
    } else {
        return x * fact(x - 1);
    }
}
/*
fn main() {
    let x = 6;
    let y = exercise::q3::fact(x);
    println!("{y}");
}
*/
