use std::io::{self, Write};

struct ToDoItem {
    name: String,
    status: bool,
    due: String,
    description: String,
    subitems: Vec<ToDoItem>,
}

fn main() {
    loop {
        let mut input = String::new();

        print!(">>> ");
        io::stdout().flush().expect("Failed to flush stdout!");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input = input.trim();

        match input {
            "exit" => break,
            _ => (),
        }

        println!("You entered: {}", input);
    }

    println!("Thanks for using the To Do App");
}
