use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::io;

pub enum Action {
    Quit,
    ScrollDown,
    ScrollUp,
    PageDown,
    PageUp,
    GoToEnd,
    OpenFilter,
    Confirm,
    None,
}

pub fn poll_input() -> io::Result<Action> {
    let mut last = Action::None;

    while event::poll(std::time::Duration::ZERO)? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            last = match key.code {
                KeyCode::Char('q') => Action::Quit,
                KeyCode::Char('j') | KeyCode::Down => Action::ScrollDown,
                KeyCode::Char('k') | KeyCode::Up => Action::ScrollUp,
                KeyCode::Char('G') => Action::GoToEnd,
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::PageDown,
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::PageUp,
                KeyCode::Char('/') => Action::OpenFilter,
                KeyCode::Enter => Action::Confirm,
                _ => continue,
            };
        }
    }

    Ok(last)
}
