use std::sync::Mutex;

use topcoat::context::{Cx, app_context};

#[derive(Clone)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub done: bool,
}

// A shared, in-memory todo list, registered as app context.
pub struct TodoList(Mutex<TodoListState>);

struct TodoListState {
    todos: Vec<Todo>,
    next_id: u64,
}

impl TodoList {
    pub fn new() -> Self {
        Self(Mutex::new(TodoListState {
            todos: Vec::new(),
            next_id: 1,
        }))
    }

    pub fn snapshot(&self) -> Vec<Todo> {
        self.0.lock().unwrap().todos.clone()
    }

    pub fn push(&self, title: &str) -> Vec<Todo> {
        let mut state = self.0.lock().unwrap();
        let id = state.next_id;
        state.next_id += 1;
        state.todos.push(Todo {
            id,
            title: title.to_owned(),
            done: false,
        });
        state.todos.clone()
    }

    pub fn toggle(&self, id: u64) -> Vec<Todo> {
        let mut state = self.0.lock().unwrap();
        if let Some(todo) = state.todos.iter_mut().find(|todo| todo.id == id) {
            todo.done = !todo.done;
        }
        state.todos.clone()
    }

    pub fn clear_done(&self) -> (Vec<Todo>, usize) {
        let mut state = self.0.lock().unwrap();
        let before = state.todos.len();
        state.todos.retain(|todo| !todo.done);
        (state.todos.clone(), before - state.todos.len())
    }
}

pub fn todos(cx: &Cx) -> &TodoList {
    app_context::<TodoList>(cx)
}
