use crate::core::clients::data::events::ClientEvents;
use crate::core::clients::data::states::ClientStates;
use crate::core::shared::reducer::Reducer;

pub struct ClientReducer {
    pub underlying: Reducer<ClientEvents, ClientStates>,
}

impl ClientReducer {
    pub fn new() -> Self {
        Self {
            underlying: Reducer {
                compute_new_state: |current, event| {
                    match current {
                        Some(current_state) => current_state.reduce_state(event),
                        None => ClientStates::reduce_state_from_empty(event)
                    }
                }
            }
        }
    }
}