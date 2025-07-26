pub fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn fibonacci_iterative(n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev: u128 = 0;
    let mut curr: u128 = 1;

    for _ in 2..=n {
        let next = prev.checked_add(curr).unwrap_or(curr);
        prev = curr;
        curr = next;
    }

    curr
}

pub fn burn_cpu(iterations: usize) -> f64 {
    let mut x = 0.5f64;
    let r = 3.9999999f64;
    for _ in 0..iterations {
        x = r * x * (1.0 - x);
    }
    x
}
