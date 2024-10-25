mod exercise;
// run test:
// cargo test -- --nocapture
/*
fn main() {
    let s: Vec<i8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let item: i8 = 8;
    exercise::q1::binary_search(s, item);
}
*/
fn main() {
    let num_slice = &[
        2, 4, 5, 12, 15, 30, 32, 33, 34, 40, 45, 51, 55, 57, 60, 66, 70, 71, 90, 99, 100,
    ];

    let result = exercise::example_1::binary_search(num_slice, 70);

    println!("Result: {:?}", result);
}
