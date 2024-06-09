pub fn factorial(n: u64) -> u64 {
    println!("current value: {}", n);
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}
