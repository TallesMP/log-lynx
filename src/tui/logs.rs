use ratatui::Frame;
use crate::log::LogEntry;
use crate::view::View;
use crate::tui::widgets::make_list;

pub fn render(frame: &mut Frame, view: &mut View, session_logs: &[LogEntry]) {
    // TODO: apply view.filter to session_logs before rendering
    frame.render_stateful_widget(
        make_list(session_logs),
        frame.area(),
        &mut view.log_list_state,
    );
}
