use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::io;

pub enum Action {
    Quit,
    ScrollDown,
    ScrollUp,
    PageDown,
    PageUp,
    GoToEnd,
    None,
}

pub fn poll_input() -> io::Result<Action> {
    if !event::poll(std::time::Duration::from_millis(16))? {
        return Ok(Action::None);
    }

    if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
            return Ok(Action::None);
        }

        return Ok(match key.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('j') => Action::ScrollDown,
            KeyCode::Char('k') => Action::ScrollUp,
            KeyCode::Char('G') => Action::GoToEnd,
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::PageDown,
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::PageUp,
            _ => Action::None,
        });
    }

    Ok(Action::None)
}
