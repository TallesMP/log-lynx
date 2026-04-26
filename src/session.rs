use std::io;
use crate::device::Device;
use crate::log::{LogEntry, LogReader};

pub struct Session {
    pub reader: LogReader,
    pub logs: Vec<LogEntry>,
}

impl Session {
    pub fn new(device: Device) -> io::Result<Self> {
        Ok(Self {
            reader: LogReader::new(device)?,
            logs: Vec::new(),
        })
    }

    pub fn load_logs(&mut self) {
        for _ in 0..100 {
            if let Some(log) = self.reader.next() {
                self.logs.push(log);
            } else {
                break;
            }
        }
    }
}
