mod todo;
mod console;

use std::io::stdin;

fn get_user_input() -> String {
    let mut option:String = String::new();
    stdin().read_line(&mut option).expect("An error occurred reading user input");
    option
}

fn main() {

    let mut store = todo::TodoStore::new();
    let console = console::Console;

    loop {

        console.show_menu();

        let option = get_user_input();

        match option.as_str().trim() {
            "1" => {
                let todos = store.get_all_todos();
                let todo_to_add = console.add_todo(todos);
                store.add_todo(todo_to_add);
            }
            "2" => {
                let id = match console.remove_todo_input() {
                    Some(id) => id,
                    None => {
                        console.remove_todo_error();
                        continue;
                    }
                };

                let has_removed_todo = store.delete_todo(id);
                console.remove_todo_result(has_removed_todo);
            }

            "3" => {
                let todos = store.get_all_todos();
                console.show_all_todos(todos);
            }
            _ => {
                println!("Unknown option, please select one of the list.");
            }

        }

    }



}
