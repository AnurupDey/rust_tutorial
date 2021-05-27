use std::io::stdin;

fn main() {
    println!("Enter N");
    let mut n = String::new();
    stdin().read_line(&mut n).expect("Input falure");
    let n: u32 = n.trim().parse().expect("Invalid Input");

    println!("{}th fibonacci number is {}", n, fib(n));
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => n + fib(n - 1),
    }
}
