use crate::core::contrats::data::{ContratEvents, ContratStates};
use crate::core::shared::reducer::Reducer;

pub struct ContratReducer {
    pub underlying: Reducer<ContratEvents, ContratStates>,
}

impl ContratReducer {
    pub fn new() -> Self {
        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    match current {
                        Some(current_state) => current_state.reduce_state(&event),
                        None => ContratStates::reduce_state_from_empty(&event)
                    }
                }
            }
        }
    }
}