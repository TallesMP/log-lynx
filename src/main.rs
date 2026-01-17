use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        Block, List, ListItem, ListState, Paragraph, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget,
    },
};
use std::io;

use crate::log::LogEntry;
mod log;

#[derive(Debug, Default)]
pub struct App {
    logs: Vec<LogEntry>,
    list_state: ListState,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut log_reader = log::LogReader::new()?;

        while !self.exit {
            while let Some(log) = log_reader.next() {
                self.logs.push(log);
            }

            terminal.draw(|frame| self.draw(frame))?;

            self.handle_event()?;
        }

        Ok(())
    }

    fn handle_event(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(char) => match char {
                        'q' => self.exit = true,
                        'j' => self.list_state.select_next(),
                        'k' => self.list_state.select_previous(),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [border_area] = Layout::vertical([Constraint::Fill(1)]).areas(frame.area());
        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);

        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .fg(Color::Blue)
            .render(border_area, frame.buffer_mut());

        let list = List::new(self.logs.iter().map(|l| ListItem::from(l.message.clone())))
            .fg(Color::Yellow)
            .highlight_style(Style::default().fg(Color::Red));

        frame.render_stateful_widget(list, inner_area, &mut self.list_state);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

