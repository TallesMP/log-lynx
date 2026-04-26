use ratatui::widgets::ListState;

// TODO: View should own Session directly, allowing App to work with Views only.
// Currently session_index is used because App needs to mutate Session (via load_logs)
// while Views hold references — Rust ownership prevents both simultaneously without Arc.
pub struct View {
    pub session_index: usize,
    pub filter: Option<String>,
    pub log_list_state: ListState,
}

impl View {
    pub fn new(session_index: usize) -> Self {
        Self {
            session_index,
            filter: None,
            log_list_state: ListState::default(),
        }
    }

    pub fn auto_scroll(&mut self, prev_len: usize) {
        if prev_len > 0 && self.log_list_state.selected() == Some(prev_len - 1) {
            self.log_list_state.select_last();
        }
    }
}
