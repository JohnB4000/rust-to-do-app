use std::io::{self, Write};

struct ToDoList {
    todo_items: Vec<ToDoItem>,
}

struct ToDoItem {
    name: String,
    status: bool,
    due: String,
    description: String,
    subitems: Vec<ToDoItem>,
}

fn print_todo_list(todo_list: &ToDoList) {
    let (longest_name, longest_due, longest_desc) = calculate_lengths(&todo_list.todo_items, 0);
    let total = longest_name + 3 + longest_due + 3 + longest_desc + 4;
    print_divider(total);
    print_header(longest_name, longest_due, longest_desc);
    print_todo_items(
        &todo_list.todo_items,
        longest_name,
        longest_due,
        longest_desc,
        0,
    );
    print_divider(total);
}

fn calculate_lengths(todos: &Vec<ToDoItem>, indent: i32) -> (i32, i32, i32) {
    let mut longest_name: i32 = 4;
    let mut longest_due: i32 = 3;
    let mut longest_desc: i32 = 11;

    for index in 0..(todos.len()) {
        let current_name_width = todos[index].name.len() as i32
            + (indent * 4)
            + (index + 1).to_string().len() as i32
            + 6;
        let current_due_width = todos[index].due.len() as i32;
        let current_desc_width = todos[index].description.len() as i32;
        if current_name_width > longest_name {
            longest_name = current_name_width
        }
        if current_due_width > longest_due {
            longest_due = current_due_width
        }
        if current_desc_width > longest_desc {
            longest_desc = current_desc_width
        }
        let (sub_name, sub_due, sub_desc) = calculate_lengths(&todos[index].subitems, indent + 1);
        if sub_name > longest_name {
            longest_name = sub_name;
        }
        if sub_due > longest_due {
            longest_due = sub_due;
        }
        if sub_desc > longest_desc {
            longest_desc = sub_desc;
        };
    }
    (longest_name, longest_due, longest_desc)
}

fn print_divider(length: i32) {
    print_char_n_times('-', length);
    println!()
}

fn print_char_n_times(char: char, times: i32) {
    for _ in 0..times {
        print!("{}", char);
    }
}

fn print_header(longest_name: i32, longest_due: i32, longest_desc: i32) {
    print!("| Name ");
    print_char_n_times(' ', longest_name - 4);
    print!("| Due ");
    print_char_n_times(' ', longest_due - 3);
    print!("| Description ");
    print_char_n_times(' ', longest_desc - 11);
    println!("|");
}

fn print_todo_items(
    todos: &Vec<ToDoItem>,
    longest_name: i32,
    longest_due: i32,
    longest_desc: i32,
    indent: i32,
) {
    for index in 0..todos.len() {
        print_divider(longest_name + longest_due + longest_desc + 10);
        print_todo_item(
            &todos[index],
            longest_name,
            longest_due,
            longest_desc,
            index as i32 + 1,
            indent,
        );
        print_todo_subitems(
            &todos[index].subitems,
            longest_name,
            longest_due,
            longest_desc,
            indent + 1,
        );
    }
}

fn print_todo_subitems(
    todos: &Vec<ToDoItem>,
    longest_name: i32,
    longest_due: i32,
    longest_desc: i32,
    indent: i32,
) {
    for index in 0..todos.len() {
        print_todo_item(
            &todos[index],
            longest_name,
            longest_due,
            longest_desc,
            index as i32 + 1,
            indent,
        );
        print_todo_subitems(
            &todos[index].subitems,
            longest_name,
            longest_due,
            longest_desc,
            indent + 1,
        );
    }
}

fn print_todo_item(
    item: &ToDoItem,
    longest_name: i32,
    longest_due: i32,
    longest_desc: i32,
    index: i32,
    indent: i32,
) {
    print!("| ");
    print_char_n_times(' ', indent * 4);
    print!(
        "{}. [{}] {} ",
        index,
        if item.status { 'x' } else { ' ' },
        item.name
    );
    print_char_n_times(
        ' ',
        longest_name
            - (item.name.len() as i32
                + index.to_string().len() as i32
                + (indent * 4)
                + (index + 1).to_string().len() as i32
                + 5),
    );
    print!("| {} ", item.due);
    print_char_n_times(' ', longest_due - item.due.len() as i32);
    print!("| {} ", item.description);
    print_char_n_times(' ', longest_desc - item.description.len() as i32);
    println!("|");
}

fn get_nth_todo(todo_list: &mut ToDoList, index: String) -> Option<&mut ToDoItem> {
    let mut index_iter = index.split(".");

    let mut parent = match index_iter.next() {
        Some(index) => match index.parse::<usize>() {
            Ok(number) => &mut todo_list.todo_items[number - 1],
            Err(_) => {
                println!("Invalid index");
                return None;
            }
        },
        None => {
            println!("Invalid index");
            return None;
        }
    };

    loop {
        let next_index = match index_iter.next() {
            Some(index) => match index.parse::<usize>() {
                Ok(number) => number - 1,
                Err(_) => {
                    println!("Invalid index");
                    return None;
                }
            },
            None => return Some(parent),
        };

        if next_index - 1 < parent.subitems.len() {
            parent = &mut parent.subitems[next_index - 1];
        } else {
            println!("Invalid index");
            return None;
        }
    }
}

fn main() {
    let mut todo_list: ToDoList = ToDoList {
        todo_items: Vec::new(),
    };
    loop {
        println!();
        println!();
        let mut input = String::new();
        print_todo_list(&todo_list);
        print!(">>> ");
        io::stdout().flush().expect("Failed to flush stdout!");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let mut input = input.trim().split_whitespace();
        let command = input.next().unwrap_or("");
        match command {
            "exit" => break,
            "add" => {
                let name = match input.next() {
                    Some(name) => name.to_string(),
                    None => {
                        println!("Please enter a name for the to do item!");
                        continue;
                    }
                };
                todo_list.todo_items.push(ToDoItem {
                    name,
                    status: false,
                    due: input.next().unwrap_or("").to_string(),
                    description: input.next().unwrap_or("").to_string(),
                    subitems: Vec::new(),
                });
            }
            "addsub" => {
                let index = match input.next() {
                    Some(index) => index.to_string(),
                    None => {
                        println!("Invalid index!");
                        continue;
                    }
                };

                let name = match input.next() {
                    Some(name) => name.to_string(),
                    None => {
                        println!("Please enter a name for the to do item!");
                        continue;
                    }
                };

                let parent = match get_nth_todo(&mut todo_list, index) {
                    None => continue,
                    Some(parent) => parent,
                };
                parent.subitems.push(ToDoItem {
                    name,
                    status: false,
                    due: input.next().unwrap_or("").to_string(),
                    description: input.next().unwrap_or("").to_string(),
                    subitems: Vec::new(),
                });
            }
            "check" | "uncheck" => {
                let index = match input.next() {
                    Some(index) => index.to_string(),
                    None => {
                        println!("Invalid index!");
                        continue;
                    }
                };

                if let Some(todo_item) = get_nth_todo(&mut todo_list, index) {
                    todo_item.status = command == "check";
                }
            }
            _ => continue,
        }
    }

    println!("Thanks for using the to do app");
}
