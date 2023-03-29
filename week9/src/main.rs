use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

pub struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Todo {
        Todo {
            map: HashMap::new(),
        }
    }

    fn add(&mut self, key: String) {
        self.map.insert(key, false);
    }

    fn complete(&mut self, key: &String) -> bool {
        match self.map.get_mut(key) {
            Some(v) => {
                *v = true;
                true
            }
            None => false,
        }
    }

    fn remove(&mut self, key: &String) -> bool {
        self.map.remove(key).is_some()
    }

    fn show(&self) {
        for (task, status) in &self.map {
            let status = if *status { "[x]" } else { "[ ]" };
            println!("{} {}", status, task);
        }
    }
}

fn main() {
    let mut todo = Todo::new();

    loop {
        println!("\nEnter a command:");
        println!("1. Add task");
        println!("2. Complete task");
        println!("3. Remove task");
        println!("4. Show tasks");
        println!("5. Exit");

        let mut input = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u32>() {
            Ok(1) => {
                println!("\nEnter task description:");
                let mut task = String::new();
                stdout().flush().unwrap();
                stdin().read_line(&mut task).unwrap();
                todo.add(task.trim().to_string());
            }
            Ok(2) => {
                println!("Enter task description:");
                let mut task = String::new();
                stdout().flush().unwrap();
                stdin().read_line(&mut task).unwrap();
                if todo.complete(&task.trim().to_string()) {
                    println!("Task completed!");
                } else {
                    println!("Task not found.");
                }
            }
            Ok(3) => {
                println!("Enter task description:");
                let mut task = String::new();
                stdout().flush().unwrap();
                stdin().read_line(&mut task).unwrap();
                if todo.remove(&task.trim().to_string()) {
                    println!("Task removed!");
                } else {
                    println!("Task not found.");
                }
            }
            Ok(4) => todo.show(),
            Ok(5) => break,
            _ => println!("\nInvalid command"),
        }
    }
}
