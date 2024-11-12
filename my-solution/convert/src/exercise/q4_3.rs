pub fn max(list: &[i32], sub_max: i32) -> i32 {
    if list.is_empty() {
        return sub_max;
    } else {
        let sub_max = max(&list[1..], sub_max);
        if sub_max > list[0] {
            return sub_max;
        } else {
            return list[0];
        }
    }
}
/*
fn main() {
    let a = &[1, 2, 3, 4, 5, 34, 12];
    let b = exercise::q4_3::max(a, a[0]);
    println!("{b}");
}
*/
