use std::io;

fn main() {
    println!("Enter temperature:");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Could not read input");
    let x: f64 = x.trim().parse().expect("Could not parse Temperature");

    println!("press 1 for C->F and 2 for F->C");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read input");
    let option: u32 = option.trim().parse().expect("Could not parse option");

    let y = match option {
        1 => (x * 9.0 / 5.0) + 32.0,
        2 => (x - 32.0) * 5.0 / 9.0,
        _ => panic!("Invalid Option"),
    };

    println!("{}", y);
}
