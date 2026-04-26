use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use crate::device::Device;

pub fn render(frame: &mut Frame, devices: &[Device], list_state: &mut ListState) {
    let area = frame.area();

    let [_, center, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Max(60),
        Constraint::Fill(1),
    ])
    .areas(area);

    let [_, center, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Max(devices.len() as u16 + 4),
        Constraint::Fill(1),
    ])
    .areas(center);

    let items: Vec<ListItem> = devices
        .iter()
        .map(|d| {
            ListItem::new(Line::from(vec![
                Span::styled(&d.model, Style::default().fg(Color::White).bold()),
                Span::raw("  "),
                Span::styled(&d.serial, Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Select device ")
                .title_style(Style::default().fg(Color::White).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, center, list_state);
}
