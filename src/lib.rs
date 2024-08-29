use std::io;

pub enum Command {
    Quit,
    Add(String),
    List,
    Complete(u32),
    Delete(u32),
    Help,
}

impl Command {
    pub fn build(input: &String) -> Result<Command, &'static str> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            return Err("No Command given");
        }

        let action = parts[0].to_lowercase();
        let argument = parts.get(1).map(|&s| s.to_string());

        match action.as_str() {
            "quit" => Ok(Command::Quit),
            "add" => argument.map(Command::Add).ok_or("Missing argument for add command"),
            "list" => Ok(Command::List),
            "complete" => {
                argument
                    .ok_or("Missing argument for complete command")
                    .and_then(|arg| arg.parse::<u32>().map(Command::Complete).map_err(|_| "Invalid number for complete command"))
            }
            "delete" => {
                argument
                    .ok_or("Missing argument for delete command")
                    .and_then(|arg| arg.parse::<u32>().map(Command::Delete).map_err(|_| "Invalid number for delete command"))
            }
            "help" => Ok(Command::Help),
            _ => Err("Invalid command given"),
        }
    }
}
pub fn process(command: Command) -> Result<(), &'static str> {
    match command {
        Command::Quit => {
            println!("Quitting...");
            std::process::exit(0);
        }
        Command::Add(task) => println!("Added task: {}", task),
        Command::List => println!("Listing tasks..."),
        Command::Complete(index) => println!("Completed task at index: {}", index),
        Command::Delete(index) => println!("Deleted task at index: {}", index),
        Command::Help => display_help(),
    }
    Ok(())
}

pub fn get_input() -> Result<String, &'static str> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(_) => Err("Failed to read input"),
    }
}

fn display_help() {
    println!("TODO CLI Help:");
    println!();
    println!("Available commands:");
    println!("  quit                - Quit the application.");
    println!("  add <task>          - Add a new task to the list.");
    println!("  list                - List all tasks.");
    println!("  complete <index>    - Mark the task at the given index as complete.");
    println!("  delete <index>      - Delete the task at the given index.");
    println!("  help                - Show this help message.");
    println!();
    println!("Examples:");
    println!("  add \"Buy groceries\"  - Adds a new task with the description 'Buy groceries'.");
    println!("  complete 1          - Marks the task at index 1 as complete.");
    println!("  delete 2            - Deletes the task at index 2.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
