fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    another_function(five());
}

fn another_function(x: u32) {
    println!("The value of x is {}", x);
}

fn five() -> u32 {
    5
}
