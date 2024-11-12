pub fn sum(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    } else {
        return &list[0] + sum(&list[1..]);
    }
}

/*
fn main() {
    let a = &[1, 2, 3, 4, 5];
    let b = exercise::q4_1::sum(a);
    println!("{b}");
}
*/
