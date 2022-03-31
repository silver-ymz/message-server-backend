pub struct State {
    pub visit_count: u64,
}

impl State {
    pub fn new() -> Self {
        State { visit_count: 0 }
    }
}
