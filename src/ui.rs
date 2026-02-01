use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Widget},
};

use crate::log::LogEntry;

fn get_level_color(level: &str) -> Color {
    match level {
        "E" => Color::Red,
        "W" => Color::Yellow,
        "I" => Color::Green,
        "D" => Color::Cyan,
        "V" | _ => Color::White,
    }
}

fn pad_center(text: &str, width: usize) -> String {
    if text.len() >= width {
        return text[..width].to_string();
    }
    let total_padding = width - text.len();
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;
    format!(
        "{}{}{}",
        " ".repeat(left_padding),
        text,
        " ".repeat(right_padding)
    )
}

fn truncate(text: &str, width: usize) -> String {
    if text.len() > width {
        text[..width].to_string()
    } else {
        format!("{:<width$}", text)
    }
}

fn get_tag_color(tag: &String) -> Color {
    let mut int_color = 0u32;

    for char in tag.chars() {
        int_color += char as u32;
    }

    return Color::from_u32(int_color);
}

pub fn render(frame: &mut Frame, logs: &[LogEntry], list_state: &mut ListState) -> usize {
    let [border_area] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .fg(Color::Blue)
        .render(border_area, frame.buffer_mut());

    let items: Vec<ListItem> = logs
        .iter()
        .map(|log| {
            let level_color = get_level_color(&log.level);
            let tag_color = get_tag_color(&log.tag);
            let pid = log.pid.unwrap_or(0);
            let tid = log.tid.unwrap_or(0);
            let package = log.package.as_deref().unwrap_or("");

            let pid_tid = format!("{}-{}", pid, tid);
            let pid_tid_padded = pad_center(&pid_tid, 11);
            let package_padded = truncate(package, 36);
            let tag_padded = truncate(&log.tag, 24);
            let level_padded = pad_center(&log.level, 3);

            let line = Line::from(vec![
                Span::styled(&log.date, Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(&log.time, Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(pid_tid_padded, Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(tag_padded, Style::default().fg(tag_color)),
                Span::raw(" "),
                Span::styled(package_padded, Style::default().fg(Color::White)),
                Span::raw(" "),
                Span::styled(level_padded, Style::default().fg(level_color)),
                Span::raw(" "),
                Span::styled(&log.message, Style::default().fg(level_color)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_stateful_widget(list, inner_area, list_state);

    inner_area.height as usize
}
