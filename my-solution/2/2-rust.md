# convert to rust

```rust
pub fn selectionsort(mut arr: Vec<u8>) {
    let mut newarr = Vec::new();
    let mut smallest = 0;
    for _ in 0..arr.len() {
        smallest = findsmallest(&arr);
        newarr.push(arr.remove(smallest));
    }
    println!("{:?}", newarr);
}
fn findsmallest(arr: &Vec<u8>) -> usize {
    let mut smallest = arr[0];
    let mut smallest_index: usize = 0;
    for i in 1..arr.len() {
        if arr[i] < smallest {
            smallest = arr[i];
            smallest_index = i;
        }
    }
    smallest_index
}

#[cfg(test)]
mod tests {
    use super::selectionsort;
    #[test]
    fn test() {
        let a: Vec<u8> = vec![6, 5, 4, 7, 8, 6, 8];
        selectionsort(a);
    }
}
// in ~/src/main.rs
/*
fn main() {
    let a: Vec<u8> = vec![3, 4, 6, 2, 3, 8, 56, 8, 96, 9];
    println!("The array before sorting is {:?}", &a);
    exercise::q2::selectionsort(a);
}
*/
```
