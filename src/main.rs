use std::io;
use std::io::Write;

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



struct Command {
    action: String,
    argument: Option<String>,
}

impl Command {
    fn build(input: &String) -> Result<Command, &'static str> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err("No Command given");
        }

        let action = parts[0].to_string();
        let argument = if parts.len() > 1 {
            Some(parts[1].to_string())
        } else {
            None
        };

        Ok(Command { action, argument })
    }
}
fn process(command: Command) -> Result<(), &'static str> {
    println!("Action: {}", command.action);
    let arg = command.argument.unwrap_or_else(|| String::new());
    println!("Argument: {}", arg);
    Ok(())
}

fn get_input() -> Result<String, &'static str> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(_) => Err("Failed to read input"),
    }
}
