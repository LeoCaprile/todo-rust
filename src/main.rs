use std::io::stdin;

fn get_user_input() -> String {
    let mut option:String = String::new();
    stdin().read_line(&mut option).expect("An error occurred reading user input");
    option
}

struct Todo {
    id: usize,
    title: String,
    description: String
}

struct TodoStore {
    todos: Vec<Todo>
}

impl TodoStore {
    fn show_all_todos(&self){

        if self.todos.len() > 0 {
            println!("----------------------------------");
            println!("-----------List of Todos----------");
            println!("----------------------------------");
            self.todos.iter().for_each(|todo| {
                println!("id: {}", todo.id);
                println!("title: {}", todo.title);
                println!("description: {}", todo.description);
                println!("----------------------------------");
            });

            println!("----Press any key to go back-----");

            let _ = get_user_input();
            return;
        }
        println!("----------------------------------");
        println!("---------------Empty--------------");
        println!("----------------------------------");
        println!("----Press any key to go back-----");

        let _ = get_user_input();
    }

    fn delete_todo(&mut self) {
        println!("----------------------------------");
        println!("---------Removing a todo----------");
        println!("--Provide the id  to remove-------");

        let user_provided_id = match get_user_input().trim().parse::<usize>() {
            Ok(id) => {
                println!("the id to remove is {id}");
                id
            },
            Err(_) => {
                println!("The provided id must be a number");
                return;
            }
        };


        match self.todos.iter().position(|todo| {
            todo.id == user_provided_id
        }) {
            None => {
                println!("The provided id doesn't exist in the list.");
                println!("----------------------------------");

            }
            Some(index) => {
                self.todos.remove(index);
                println!("--Todo removed with success!------");
                println!("----------------------------------");
            }
        }

    }

    fn add_todo(&mut self){
        println!("----------------------------------");
        println!("---------Adding a todo------------");
        println!("--Provide a title-----------------");
        let title = get_user_input().trim().to_string();
        println!("--Provide a description-----------");
        let description = get_user_input().trim().to_string();
        println!("----------------------------------");
        let index = self.todos.len();

        let todo_to_add = Todo {
            id: index,
            title,
            description,
        };

        self.todos.insert(index, todo_to_add);
        println!("--Todo added with success!--------");
        println!("----------------------------------");

    }

}

fn main() {

    let mut store = TodoStore{ todos: vec![] };
    loop {
        print!("\x1B[2J\x1B[1;1H");

        println!("----------------------------------");
        println!("---Welcome to Todo App in rust ---");
        println!("----------------------------------");

        println!("-Select your action--------------");
        println!("1. Add a todo");
        println!("2. Remove a todo");
        println!("3. Show all todos");
        println!("----------------------------------");

        let option = get_user_input();

        println!("option: {:?}", option);

        match option.as_str().trim() {
            "1" => {
                print!("\x1B[2J\x1B[1;1H");
                store.add_todo();
            }
            "2" => {
                print!("\x1B[2J\x1B[1;1H");
                store.delete_todo();
            }

            "3" => {
                print!("\x1B[2J\x1B[1;1H");
                store.show_all_todos();
            }
            _ => {
                println!("Unknown option, please select one of the list.");
            }

        }



    }



}
