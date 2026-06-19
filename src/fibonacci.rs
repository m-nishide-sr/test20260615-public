// フィボナッチ(再帰)
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// フィボナッチ(ループ化)
pub fn fibonacci_loop(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    let mut a = 1;
    let mut b = 1;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}