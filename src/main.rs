fn helper_fib(n:i64, a:i64, b:i64) -> i64 {
    if n == 0 {
        return a;
    } else if n == 1 {
        return b;
    }
    return helper_fib(n - 1, b, a+b);
}

fn fibonacci(n:i64) -> i64 {
    return helper_fib(n, 0, 1);
}

fn main() {
    for i in 0..93i64 {
        println!("{}", fibonacci(i));
    }
}
