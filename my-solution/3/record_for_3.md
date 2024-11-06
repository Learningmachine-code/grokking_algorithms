# in q3.rs

```rust
pub fn fact(x: u32) -> u32 {
    if x == 1 {
        return 1;
    } else {
        return x * fact(x - 1);
    }
}
```

```rust
fn main() {
    let x = 6;
    let y = exercise::q3::fact(x);
    println!("{y}");
}
```

## Q 3.2

>Suppose you accidentally write a recursive function that runs forever. As you saw, your computer allocates memory on the stack for each function call. What happens to the stack when your recursive function runs forever?

## A 3.2

>The memory of the computer will be full;
