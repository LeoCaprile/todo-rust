use crate::get_user_input;
use crate::todo::Todo;

pub struct Console;

impl Console {

    pub fn clear(&self){
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn stop_print(&self){
        println!("----Press any key to go back-----");

        let _ = get_user_input();
    }

    pub fn show_menu(&self){
        self.clear();

        println!("----------------------------------");
        println!("---Welcome to Todo App in rust ---");
        println!("----------------------------------");

        println!("-Select your action--------------");
        println!("1. Add a todo");
        println!("2. Remove a todo");
        println!("3. Show all todos");
        println!("----------------------------------");
    }

    pub fn add_todo(&self, todos: &Vec<Todo>) -> Todo {
        self.clear();

        println!("----------------------------------");
        println!("---------Adding a todo------------");
        println!("--Provide a title-----------------");

        let title = get_user_input().trim().to_string();
        println!("--Provide a description-----------");
        let description = get_user_input().trim().to_string();
        println!("----------------------------------");
        let index = todos.len();

        println!("--Todo added with success!--------");
        println!("----------------------------------");

        let todo_to_add = Todo {
            id: index,
            title,
            description,
        };

        todo_to_add
    }

    pub fn remove_todo_input(&self) -> Option<usize> {
        self.clear();

        println!("----------------------------------");
        println!("---------Removing a todo----------");
        println!("--Provide the id  to remove-------");

        let user_provided_id = match get_user_input().trim().parse::<usize>() {
            Ok(id) => Some(id),
            Err(_) => None
        };

        user_provided_id

    }

    pub fn remove_todo_error(&self) {
        println!("-The provided ID must be a number-");
        self.stop_print();
    }

    pub fn remove_todo_result(&self, result: bool) {
        if result {
            println!("--Todo removed with success!------");
            println!("----------------------------------");
        } else {
            println!("The provided id doesn't exist in the list.");
            println!("----------------------------------");
        }
        self.stop_print();
    }

    pub fn show_all_todos(&self ,todos: &Vec<Todo>){
        self.clear();

        if todos.len() > 0 {
            println!("----------------------------------");
            println!("-----------List of Todos----------");
            println!("----------------------------------");
            todos.iter().for_each(|todo| {
                println!("{:?}", todo);
                println!("----------------------------------");
            });
        self.stop_print();
        } else {
            println!("----------------------------------");
            println!("---------------Empty--------------");
            println!("----------------------------------");
            self.stop_print();
        }

    }

}