// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// use ratatui::{
//     DefaultTerminal, Frame,
//     buffer::Buffer,
//     layout::Rect,
//     style::Stylize,
//     symbols::border,
//     text::{Line, Text},
//     widgets::{Block, Paragraph, Widget},
// };
// use std::io;
// mod log;
//
// #[derive(Debug, Default)]
// pub struct App {
//     exit: bool,
// }
//
// impl App {
//     pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
//         terminal.draw(|frame| self.draw(frame))?;
//         self.handle_events()?;
//         Ok(())
//     }
//
//     fn draw(&self, frame: &mut Frame) {
//         Paragraph::new("Hello world teste").render(frame.area(), frame.buffer_mut());
//     }
//
//     fn handle_events(&mut self) -> io::Result<()> {
//         loop {
//             if let Event::Key(key) = event::read()? {
//                 match key.code {
//                     event::KeyCode::Esc => {
//                         break;
//                     }
//                     _ => {}
//                 }
//             }
//         }
//         Ok(())
//     }
// }
//
// fn main() -> io::Result<()> {
//     let mut terminal = ratatui::init();
//     let app_result = App::default().run(&mut terminal);
//     ratatui::restore();
//     app_result
// }
//
mod log;
// fn main() {
//     let log_reader = match log::LogReader::new() {
//         Ok(reader) => reader,
//         Err(err) => {
//             eprintln!("Erro ao criar LogReader: {}", err);
//             return;
//         }
//     };
//
//     // Inicia o processo de leitura dos logs
//     log_reader.start();
//     println!("iniciado");
//     loop {
//         if let Some(log) = log_reader.next() {
//             // Exibir ou processar o log
//             println!("{:?}", log);
//         }
//     }
// }
fn main() -> std::io::Result<()> {
    let mut log_reader = log::LogReader::new()?;

    println!("Iniciando leitura de logs...");

    loop {
        if let Some(entry) = log_reader.next() {
            println!("{:?}", entry);
            // ou faça o que quiser com a entrada
        }
        // O loop bloqueia até chegar uma nova linha do logcat
        // Se quiser adicionar um pequeno delay ou tratamento de interrupção, pode envolver com std::thread::sleep ou ctrl-c handler
    }
}
