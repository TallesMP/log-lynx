use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame) {
    let area = frame.area();

    let [_, center, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Max(40),
        Constraint::Fill(1),
    ])
    .areas(area);

    let [_, center, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Max(3),
        Constraint::Fill(1),
    ])
    .areas(center);

    frame.render_widget(
        Paragraph::new("- waiting for device -")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray)),
            ),
        center,
    );
}
