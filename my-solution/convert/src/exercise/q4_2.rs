pub fn count(list: &[i32], counted: i32) -> i32 {
    if list.is_empty() {
        return counted;
    } else {
        return count(&list[1..], counted + 1);
    }
}
/*
fn main() {
    let a = &[1, 2, 3, 4, 5, 34, 12];
    let b = exercise::q4_2::count(a, 0);
    println!("{b}");
}
*/
