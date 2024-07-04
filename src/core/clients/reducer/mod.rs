use crate::core::shared::reducer::Reducer;
use crate::core::clients::data::{ClientData, ClientEvents, ClientStates};
use crate::core::clients::data::ClientEvents::{Created, Updated};

pub struct ClientReducer {
    pub underlying: Reducer<ClientEvents, ClientStates>,
}

impl ClientReducer {
    pub fn new() -> Self {
        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    match current {
                        Some(current_state) => {
                            match event {
                                Updated(e) =>
                                    Some(
                                        ClientStates::Client({
                                            let client_data = current_state.data();
                                            let last_name = client_data.last_name;
                                            let birth_date = client_data.birth_date;
                                            ClientData { first_name: e.name, last_name, birth_date }
                                        })
                                    ),
                                _ => None
                            }
                        }
                        None => {
                            match event {
                                Created { by: _, at: _, first_name, last_name, birth_date } =>
                                    Some(ClientStates::Client(ClientData { first_name, last_name, birth_date })),
                                _ => None
                            }
                        }
                    }
                }
            }
        }
    }
}