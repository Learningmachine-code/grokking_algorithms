# about test

```rust
mod exercise;
// run test:
// cargo test -- --nocapture
fn main() {
    let s: Vec<i8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let item: i8 = 8;
    exercise::q1::binary_search(s, item);
}
```

> run test:
> cargo test -- --nocapture

```rust
pub fn binary_search(s: Vec<i8>, item: i8) {
    let mut low: i8 = 0;
    let mut high: i8 = s.len() as i8 - 1;
    let mut mid: i8;
    let mut guess: i8;
    while low <= high {
        mid = (low + high) / 2;
        guess = s[mid as usize];
        if guess == item {
            println!("The index of the item is {mid}");
            return;
        }
        if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    println!("There is None!")
}
#[cfg(test)]
mod tests {
    use super::binary_search;
    #[test]
    fn test() {
        let s: Vec<i8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let item: i8 = 8;
        binary_search(s, item);
    }
}
```
