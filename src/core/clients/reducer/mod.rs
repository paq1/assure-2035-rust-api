use crate::core::shared::reducer::Reducer;
use crate::core::clients::data::{ClientEvents, ClientStates};
use crate::core::clients::data::ClientEvents::{Created, Updated};

pub struct ClientReducer {
    pub underlying: Reducer<ClientEvents, ClientStates>
}

impl ClientReducer {
    pub fn new() -> Self {

        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    if current.is_none() {
                        match event {
                            Created { by: _, at: _, name} => Some(ClientStates::Client { name }),
                            _ => None
                        }
                    } else {
                        match event {
                            Updated (e) => Some(ClientStates::Client {name: e.name}),
                            _ => None
                        }
                    }
                }
            }
        }
    }
}