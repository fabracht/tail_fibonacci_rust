fn helper_fib(n:i64, a:i64, b:i64) -> i64 {
    if n == 0 {
        return a;
    } else if n == 1 {
        return b;
    }
    return helper_fib(n - 1, b, a+b);
}

fn fibonacci(n:i64) -> i64 {
    helper_fib(n, 0, 1)
}

fn print_fibo(n: usize) {
    let ordinal = match n {
        1 => format!("{}st", n),
        2 => format!("{}nd", n),
        3 => format!("{}rd", n),
        _ => format!("{}th", n),
    };
    println!("The {} fibonacci number is {}", ordinal, fibonacci(n as i64));
}

fn main() {
    for i in 0..93 {
        print_fibo(i);
    }
}
