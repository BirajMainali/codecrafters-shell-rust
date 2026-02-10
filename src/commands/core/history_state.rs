use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub struct HistoryState {
    pub commands: Vec<String>,
    pub last_appended_index: usize,
}

pub type SharedHistory = Arc<Mutex<HistoryState>>;
