use ratatui;
use std::io;

mod input;
mod log;
mod session;
mod app;
mod device;
mod view;

mod tui {
    pub mod logs;
    pub mod devices;
    pub mod waiting;
    pub mod widgets;
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = app::App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
