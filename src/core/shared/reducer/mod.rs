pub struct Reducer<EVENT, STATE> {
    pub compute_new_state: fn(Option<STATE>, EVENT) -> Option<STATE>,
}
