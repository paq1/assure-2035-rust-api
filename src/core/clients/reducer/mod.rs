use crate::core::shared::reducer::Reducer;
use crate::core::clients::data::{ClientActif, ClientEvents, ClientStates};
use crate::core::clients::data::ClientEvents::{Created, Updated};
use crate::models::clients::shared::ClientData;

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
                                            ClientActif {
                                                kind: "org:example:insurance:client".to_string(),
                                                data: ClientData { first_name: e.name, last_name, birth_date },
                                            }
                                        })
                                    ),
                                _ => None
                            }
                        }
                        None => {
                            match event {
                                Created(data) =>
                                    Some(
                                        ClientStates::Client(
                                            ClientActif {
                                                kind: "org:example:insurance:client".to_string(),
                                                data: ClientData {
                                                    first_name: data.data.first_name,
                                                    last_name: data.data.last_name,
                                                    birth_date: data.data.birth_date
                                                }
                                            }
                                        )
                                    ),
                                _ => None
                            }
                        }
                    }
                }
            }
        }
    }
}