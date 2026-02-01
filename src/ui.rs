use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, List, ListItem, ListState, Widget},
};

use crate::log::LogEntry;

pub fn render(frame: &mut Frame, logs: &[LogEntry], list_state: &mut ListState) -> usize {
    let [border_area] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);
    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::Blue)
        .render(border_area, frame.buffer_mut());

    let list = List::new(logs.iter().map(|l| ListItem::from(l.message.clone())))
        .fg(Color::Yellow)
        .highlight_style(Style::default().fg(Color::Red));

    frame.render_stateful_widget(list, inner_area, list_state);
    
    inner_area.height as usize
}
