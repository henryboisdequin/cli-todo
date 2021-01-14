use std::io;

#[derive(Debug)]
struct TodoItem {
    name: String,
    completed: char,
    index: usize,
}

impl TodoItem {
    fn new(name: String, index: usize) -> Self {
        TodoItem {
            name,
            completed: ' ',
            index,
        }
    }
}

#[derive(Debug)]
struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { list: Vec::new() }
    }

    fn add_todo(&mut self) {
        // read input
        println!("\nEnter your todo:\n");
        let mut name_of_todo = String::new();
        io::stdin().read_line(&mut name_of_todo).unwrap();
        name_of_todo = name_of_todo.split_whitespace().collect();

        // push todo item
        let todo = TodoItem::new(name_of_todo, self.list.len());
        &self.list.push(todo);
    }

    fn print_todos(&self) {
        // no todos
        if self.list.len() == 0 {
            println!("No todos.");
        }

        // loop through and print all todos
        for item in &self.list {
            println!(
                "Index: {} - [{}] - {}",
                item.index, item.completed, item.name
            );
        }
    }

    fn toggle_complete(&mut self) {
        // read input
        println!("\nEnter the index of the todo you want to toggle complete:\n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let index: usize = input.trim().parse().expect("Please enter a number");

        // set todo.completed to completed
        self.list[index].completed = 'x';
    }

    fn remove_todo(&mut self) {
        // read input
        println!("\nEnter the index of the todo you want to remove:\n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let index: usize = input.trim().parse().expect("Please enter a number");

        // remove todo from todo list
        self.list.remove(index);
    }

    fn edit_todo(&mut self) {
        // read input
        println!("\nEnter the index of the todo you want to edit:\n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let index: usize = input.trim().parse().expect("Please enter a number");

        // read edit todo name
        println!("\nEnter the name of the todo you want to update:\n");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();

        // update the name
        self.list[index].name = name;
    }
}

fn main() {
    // create a new todo list
    let mut todo_list = TodoList::new();

    // loop forever
    loop {
        // read command
        println!(
            "\nCommands:\n1: Get all todos\n2: Add a todo\n3: Toggle complete\n4: Remove todo\n5: Edit todo\n6: Quit\n"
        );
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        command = command.trim().to_string();

        // do action based on given command
        if command == "1" {
            todo_list.print_todos();
        } else if command == "2" {
            todo_list.add_todo();
        } else if command == "3" {
            todo_list.toggle_complete();
        } else if command == "4" {
            todo_list.remove_todo();
        } else if command == "5" {
            todo_list.edit_todo();
        } else if command == "6" {
            break;
        }
    }
}
