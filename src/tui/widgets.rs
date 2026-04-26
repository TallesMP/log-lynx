use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
};

use crate::log::LogEntry;

fn get_level_color(level: &str) -> Color {
    match level {
        "E" => Color::Red,
        "W" => Color::Yellow,
        "I" => Color::Green,
        "D" => Color::Cyan,
        _ => Color::White,
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
    let chars: String = text.chars().take(width).collect();
    let len = chars.chars().count();
    if len < width {
        format!("{chars:<width$}")
    } else {
        chars
    }
}

fn get_tag_color(tag: &str) -> Color {
    let int_color: u32 = tag.chars().map(|c| c as u32).sum();
    Color::from_u32(int_color)
}

fn render_log_line(log: &LogEntry) -> Line<'_> {
    let level_color = get_level_color(&log.level);
    let tag_color = get_tag_color(&log.tag);
    let pid = log.pid.unwrap_or(0);
    let tid = log.tid.unwrap_or(0);

    let pid_tid = format!("{}-{}", pid, tid);
    let pid_tid_padded = pad_center(&pid_tid, 11);
    let package_padded = truncate(log.package.as_deref().unwrap_or(""), 36);
    let tag_padded = truncate(&log.tag, 24);
    let level_padded = pad_center(&log.level, 3);

    Line::from(vec![
        Span::styled(log.date.as_str(), Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(log.time.as_str(), Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(pid_tid_padded, Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(tag_padded, Style::default().fg(tag_color)),
        Span::raw(" "),
        Span::styled(package_padded, Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(level_padded, Style::default().fg(level_color)),
        Span::raw(" "),
        Span::styled(log.message.as_str(), Style::default().fg(level_color)),
    ])
}

pub fn make_list(logs: &[LogEntry]) -> List<'_> {
    let items: Vec<ListItem> = logs
        .iter()
        .map(|log| ListItem::new(render_log_line(log)))
        .collect();

    List::new(items)
        .highlight_style(Style::default().bg(Color::DarkGray))
}
