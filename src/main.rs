use ratatui::{DefaultTerminal, widgets::ListState};
use std::io;

mod input;
mod log;
mod ui;

use crate::log::LogEntry;

#[derive(Debug, Default)]
pub struct App {
    logs: Vec<LogEntry>,
    list_state: ListState,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut log_reader = log::LogReader::new()?;
        let mut visible_lines = 0;

        while !self.exit {
            self.load_logs(&mut log_reader);
            terminal.draw(|frame| {
                visible_lines = ui::render(frame, &self.logs, &mut self.list_state);
            })?;
            self.handle_input(visible_lines)?;
        }

        Ok(())
    }

    fn load_logs(&mut self, log_reader: &mut log::LogReader) {
        for _ in 0..100 {
            if let Some(log) = log_reader.next() {
                self.logs.push(log);

                // follow next line
                if let Some(selected) = self.list_state.selected() {
                    if selected == self.logs.len() - 2 {
                        self.list_state.select_next();
                    }
                }
            } else {
                break;
            }
        }
    }

    fn handle_input(&mut self, visible_lines: usize) -> io::Result<()> {
        let half_page = (visible_lines / 2) as u16;

        match input::poll_input()? {
            input::Action::Quit => self.exit = true,
            input::Action::ScrollDown => self.list_state.select_next(),
            input::Action::ScrollUp => self.list_state.select_previous(),
            input::Action::PageDown => self.list_state.scroll_down_by(half_page),
            input::Action::PageUp => self.list_state.scroll_up_by(half_page),
            input::Action::GoToEnd => self.list_state.select_last(),
            input::Action::None => {}
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
