use std::io::{self, Write};


fn main() {
    let mut name = String::new();

    print!("What is your name? ");
    io::stdout().flush().expect("Ok");

    io::stdin().read_line(&mut name)
        .expect("You must enter your name!");

    println!("{}", saying_hello::return_greetings(name.trim()));
}
