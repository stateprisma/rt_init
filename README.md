# rt_init

A simple macro easing the use of Rust's `spin::lazy`

## Motivation

In Rust, defining static variables that require complex initialization often necessitates the use of `spin::lazy`, which can reduce the readability of the source code. To address this, we have developed a macro system called `rt_init` (short for runtime init), which simplifies the creation and initialization of multiple static variables.

## Example

```rs
rt_init! {
    static STATIC1: Vec<u64> = {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec
    };

    static STATIC2: u64 = 42;
    static STATIC3: String = "Hello, World!".to_string();
}


fn main() {
    println!("Vec   : {:?}", *STATIC1);
    println!("u64   : {:?}", *STATIC2);
    println!("String: {:?}", *STATIC3);
}
```

In this example, the `rt_init!` macro allows you to define static variables `STATIC1`, `STATIC2`, and `STATIC3` with different types and initialization logic. This approach eliminates the need for manual handling with `spin::lazy`, making the code more concise and easier to understand.
