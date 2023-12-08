mod todo;

use std::io::stdin;

fn get_user_input() -> String {
    let mut option:String = String::new();
    stdin().read_line(&mut option).expect("An error occurred reading user input");
    option
}

fn main() {

    let mut store = todo::TodoStore::new();

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
