#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: String
}

pub struct TodoStore {
    pub todos: Vec<Todo>
}

impl TodoStore {

    pub fn new() -> Self {
        TodoStore { todos: vec![] }
    }
    pub fn get_all_todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn delete_todo(&mut self, id: usize) -> bool {

        match self.todos.iter().position(|todo| {
            todo.id == id
        }) {
            None => false,
            Some(index) => {
                self.todos.remove(index);
                true
            }
        }

    }

    pub fn add_todo(&mut self, todo_to_add: Todo){
        self.todos.insert(todo_to_add.id, todo_to_add);
    }

}

#[cfg(test)]
mod tests {
    use crate::todo::{Todo, TodoStore};

    #[test]
    fn can_instantiate_todo_store() {
        let todo_store = TodoStore::new();
        assert_eq!(todo_store.todos.len(), 0);
    }

    #[test]
    fn can_add_todo_to_todo_store() {
        let mut todo_store = TodoStore::new();

        todo_store.add_todo(Todo {
            id: 0,
            title: String::from("title"),
            description: String::from("desc")
        });

        assert_eq!(todo_store.todos.len(), 1);

    }
    #[test]
    fn can_list_all_todos_from_todo_store() {
        let mut todo_store = TodoStore::new();

        todo_store.add_todo(Todo {
            id: 0,
            title: String::from("title"),
            description: String::from("desc")
        });

        todo_store.add_todo(Todo {
            id: 1,
            title: String::from("title2"),
            description: String::from("desc2")
        });

        assert_eq!(todo_store.todos.len(), 2);

        assert_eq!(todo_store.get_all_todos().len(), 2)

    }

    #[test]
    fn can_delete_todo_from_todo_store() {
        let mut todo_store = TodoStore::new();

        todo_store.add_todo(Todo {
            id: 0,
            title: String::from("title"),
            description: String::from("desc")
        });

        assert_eq!(todo_store.todos.len(), 1);

        todo_store.delete_todo(0);

        assert_eq!(todo_store.todos.len(), 0);

    }
}