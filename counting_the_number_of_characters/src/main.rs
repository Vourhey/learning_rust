use std::io::{self, Write};

fn main() {
    print!("What is the input string? ");
    io::stdout().flush().expect("Haven't done flush");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Something went wrong");

    input = input.trim().to_string();

    println!("{} has {} characters", input, input.len());
}
