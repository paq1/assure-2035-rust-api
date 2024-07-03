use crate::core::shared::reducer::Reducer;
use crate::core::todos::data::{TodoEvents, TodoStates};
use crate::core::todos::data::TodoEvents::{Created, Updated};

pub struct TodoReducer {
    pub underlying: Reducer<TodoEvents, TodoStates>
}

impl TodoReducer {
    pub fn new() -> Self {

        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    if current.is_none() {
                        match event {
                            Created { by: _, at: _, name} => Some(TodoStates::Todo { name }),
                            _ => None
                        }
                    } else {
                        match event {
                            Updated (e) => Some(TodoStates::Todo {name: e.name}),
                            _ => None
                        }
                    }
                }
            }
        }
    }
}