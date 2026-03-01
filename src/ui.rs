use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, ListState, StatefulWidget, Widget},
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
        Span::styled(log.date.clone(), Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(log.time.clone(), Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(pid_tid_padded, Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(tag_padded, Style::default().fg(tag_color)),
        Span::raw(" "),
        Span::styled(package_padded, Style::default().fg(Color::White)),
        Span::raw(" "),
        Span::styled(level_padded, Style::default().fg(level_color)),
        Span::raw(" "),
        Span::styled(log.message.clone(), Style::default().fg(level_color)),
    ])
}

struct LogList<'a> {
    logs: &'a [LogEntry],
}

impl StatefulWidget for LogList<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.is_empty() || self.logs.is_empty() {
            if self.logs.is_empty() {
                state.select(None);
            }
            return;
        }

        let len = self.logs.len();
        if state.selected().is_some_and(|s| s >= len) {
            state.select(Some(len - 1));
        }

        let height = area.height as usize;
        let (first, last) = get_visible_bounds(state.selected(), state.offset(), len, height);
        *state.offset_mut() = first;

        for (i, log) in self.logs[first..last].iter().enumerate() {
            let y = area.top() + i as u16;
            let row_area = Rect { x: area.left(), y, width: area.width, height: 1 };

            let is_selected = state.selected().map_or(false, |s| s == first + i);

            let line = render_log_line(log);
            buf.set_line(row_area.x, row_area.y, &line, row_area.width);

            if is_selected {
                buf.set_style(row_area, Style::default().bg(Color::DarkGray));
            }
        }
    }
}

fn get_visible_bounds(
    selected: Option<usize>,
    offset: usize,
    total: usize,
    height: usize,
) -> (usize, usize) {
    let offset = offset.min(total.saturating_sub(1));
    let mut first = offset;
    let mut last = (offset + height).min(total);

    if let Some(sel) = selected {
        let sel = sel.min(total - 1);
        while sel >= last {
            last = (last + 1).min(total);
            if last - first > height {
                first += 1;
            }
        }
        while sel < first {
            first -= 1;
            if last - first > height {
                last -= 1;
            }
        }
    }

    (first, last)
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

    let log_list = LogList { logs };
    frame.render_stateful_widget(log_list, inner_area, list_state);

    inner_area.height as usize
}

use ratatui::Frame;
