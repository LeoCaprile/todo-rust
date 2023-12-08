use crate::get_user_input;

struct Todo {
    id: usize,
    title: String,
    description: String
}

pub struct TodoStore {
    pub todos: Vec<Todo>
}

impl TodoStore {

    pub fn new() -> TodoStore {
        TodoStore { todos: vec![] }
    }
    pub fn show_all_todos(&self){

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

    pub fn delete_todo(&mut self) {
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

    pub fn add_todo(&mut self){
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