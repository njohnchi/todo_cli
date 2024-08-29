use std::io;
use std::io::Write;
use todo_cli::{get_input, Command, process};

fn main() {
    println!("TODO CLI");
    loop {
        print!("Todo > ");
        io::stdout().flush().unwrap();

        match get_input() {
            Ok(input) => {
                match Command::build(&input) {
                    Ok(cmd) => {
                        if let Err(err) = process(cmd) {
                            eprintln!("Processing error: {}", err);
                        }
                    },
                    Err(err) => eprintln!("Command error: {}", err),
                }
            },
            Err(err) => eprintln!("Input error: {}", err)
        }
    }
}
