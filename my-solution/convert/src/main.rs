fn main() {
    println!("Hello, world!");
    let s: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let item: usize = 11;
    let a = binary_search(s, item);
    match a {
        Some(n) => println!("The index of this list is {n}."),
        None => println!("There is none."),
    }
}

fn binary_search(s: Vec<usize>, item: usize) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = s.len() - 1;
    let mut mid: usize;
    let mut guess: usize;
    while low <= high {
        mid = (low + high) / 2;
        guess = s[mid];
        if guess == item {
            return Some(mid);
        }
        if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    return None;
}
