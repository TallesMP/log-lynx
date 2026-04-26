use std::io;
use ratatui::DefaultTerminal;
use ratatui::widgets::ListState;
use crate::session::Session;
use crate::device::{Device, ConnectedDevices};
use crate::view::View;
use crate::input::{Action, poll_input};
use crate::tui::logs;
use crate::tui::devices;
use crate::tui::waiting;

pub enum AppScreen {
    WaitingForDevice,
    DeviceSelection,
    LogView,
}

pub struct App {
    pub screen: AppScreen,
    pub sessions: Vec<Session>,
    pub tabs: Vec<View>,
    pub active_tab: usize,
    pub devices: Vec<Device>,
    pub device_list_state: ListState,
    pub exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: AppScreen::WaitingForDevice,
            sessions: Vec::new(),
            tabs: Vec::new(),
            active_tab: 0,
            devices: Vec::new(),
            device_list_state: ListState::default(),
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut visible_lines = 0;

        while !self.exit {
            self.maintain_connection()?;
            self.load_all_sessions();

            terminal.draw(|frame| {
                visible_lines = self.render(frame);
            })?;

            self.handle_input(visible_lines)?;
        }
        Ok(())
    }

    fn load_all_sessions(&mut self) {
        let changes: Vec<(usize, usize)> = self.sessions
            .iter_mut()
            .enumerate()
            .filter_map(|(i, session)| {
                let prev_len = session.logs.len();
                session.load_logs();
                (session.logs.len() > prev_len).then_some((i, prev_len))
            })
            .collect();

        for (session_idx, prev_len) in changes {
            for view in self.tabs.iter_mut().filter(|v| v.session_index == session_idx) {
                view.auto_scroll(prev_len);
            }
        }
    }

    fn render(&mut self, frame: &mut ratatui::Frame) -> usize {
        match self.screen {
            AppScreen::LogView => {
                if let Some(view) = self.tabs.get_mut(self.active_tab) {
                    let session_index = view.session_index;
                    if let Some(session) = self.sessions.get(session_index) {
                        logs::render(frame, view, &session.logs);
                    }
                }
            }
            AppScreen::DeviceSelection => {
                devices::render(frame, &self.devices, &mut self.device_list_state);
            }
            AppScreen::WaitingForDevice => waiting::render(frame),
        }
        frame.area().height as usize
    }

    fn select_device(&mut self, device: Device) -> io::Result<()> {
        let session_index = self.sessions.len();
        self.sessions.push(Session::new(device)?);
        self.tabs.push(View::new(session_index));
        self.active_tab = self.tabs.len() - 1;
        self.screen = AppScreen::LogView;
        Ok(())
    }

    fn maintain_connection(&mut self) -> io::Result<()> {
        if !self.sessions.is_empty() {
            return Ok(());
        }

        let connected = ConnectedDevices::update()?;
        let authorized: Vec<Device> = connected
            .devices
            .into_iter()
            .filter(|d| d.authorized)
            .collect();

        match authorized.len() {
            1 => {
                let device = authorized.into_iter().next().unwrap();
                self.select_device(device)?;
            }
            0 => {
                self.screen = AppScreen::WaitingForDevice;
                self.devices.clear();
            }
            _ => {
                self.devices = authorized;
                if !matches!(self.screen, AppScreen::DeviceSelection) {
                    self.screen = AppScreen::DeviceSelection;
                    self.device_list_state.select(Some(0));
                }
            }
        }

        Ok(())
    }

    fn handle_input(&mut self, visible_lines: usize) -> io::Result<()> {
        let action = match poll_input()? {
            Action::None => return Ok(()),
            Action::Quit => {
                self.exit = true;
                return Ok(());
            }
            a => a,
        };

        match self.screen {
            AppScreen::LogView => {
                if let Some(view) = self.tabs.get_mut(self.active_tab) {
                    let half_page = (visible_lines / 2) as u16;
                    match action {
                        Action::ScrollDown => view.log_list_state.select_next(),
                        Action::ScrollUp => view.log_list_state.select_previous(),
                        Action::PageDown => view.log_list_state.scroll_down_by(half_page),
                        Action::PageUp => view.log_list_state.scroll_up_by(half_page),
                        Action::GoToEnd => view.log_list_state.select_last(),
                        _ => {}
                    }
                }
            }
            AppScreen::DeviceSelection => match action {
                Action::ScrollDown => self.device_list_state.select_next(),
                Action::ScrollUp => self.device_list_state.select_previous(),
                Action::Confirm => {
                    if let Some(idx) = self.device_list_state.selected() {
                        if idx < self.devices.len() {
                            let device = self.devices.remove(idx);
                            self.devices.clear();
                            self.select_device(device)?;
                        }
                    }
                }
                _ => {}
            },
            AppScreen::WaitingForDevice => {}
        }

        Ok(())
    }
}
