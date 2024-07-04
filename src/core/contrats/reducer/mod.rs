use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::contrats::data::ContratEvents::{Created, Updated};
use crate::core::shared::reducer::Reducer;

pub struct ContratReducer {
    pub underlying: Reducer<ContratEvents, ContratStates>
}

impl ContratReducer {
    pub fn new() -> Self {

        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    if current.is_none() {
                        match event {
                            Created { by: _, at: _, name} => Some(ContratStates::Contrat { name }),
                            _ => None
                        }
                    } else {
                        match event {
                            Updated (e) => Some(ContratStates::Contrat {name: e.name}),
                            _ => None
                        }
                    }
                }
            }
        }
    }
}