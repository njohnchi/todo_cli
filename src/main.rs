use std::io;
use std::io::Write;

fn main() {
    println!("TODO CLI");
    loop {
        print!("Todo > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => println!("{}", input.trim()),
            Err(e) => println!("An error occurred: {e}"),
        }
    }
}
