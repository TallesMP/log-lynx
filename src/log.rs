use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub date: String,
    pub time: String,
    pub pid: Option<u32>,
    pub tid: Option<u32>,
    pub level: String,
    pub tag: String,
    pub package: Option<String>,
    pub message: String,
}

pub struct LogReader {
    deque: Arc<Mutex<VecDeque<LogEntry>>>,
    logcat: Option<Child>,
    shell: Option<Child>,
}

impl LogReader {
    pub fn new() -> io::Result<Self> {
        let mut logcat = Command::new("adb")
            .arg("shell")
            .arg("logcat")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = logcat
            .stdout
            .take()
            .expect("Failed to open stdout of logcat");

        let mut shell = Command::new("adb")
            .arg("shell")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let shell_stdin = shell.stdin.take().expect("Failed to open stdin");
        let shell_stdout = shell.stdout.take().expect("Failed to open stdout");

        let deque = Arc::new(Mutex::new(VecDeque::new()));
        let deque_ref = Arc::clone(&deque);

        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            let mut buffer = String::new();
            let mut package_cache = HashMap::new();
            let mut shell_stdin = shell_stdin;
            let mut shell_stdout = BufReader::new(shell_stdout);

            loop {
                buffer.clear();
                match reader.read_line(&mut buffer) {
                    Ok(0) => {
                        eprintln!("logcat stream ended");
                        break;
                    }
                    Ok(_) => {
                        let line = buffer.trim();
                        if let Some(entry) = Self::parse_line(
                            line,
                            &mut package_cache,
                            &mut shell_stdin,
                            &mut shell_stdout,
                        ) {
                            deque_ref.lock().unwrap().push_back(entry);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading logcat: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(Self {
            deque,
            logcat: Some(logcat),
            shell: Some(shell),
        })
    }

    pub fn next(&mut self) -> Option<LogEntry> {
        self.deque.lock().unwrap().pop_front()
    }

    fn get_package(
        pid: &str,
        cache: &mut HashMap<String, String>,
        stdin: &mut std::process::ChildStdin,
        stdout: &mut BufReader<std::process::ChildStdout>,
    ) -> Option<String> {
        if let Some(package) = cache.get(pid) {
            return Some(package.to_string());
        }

        writeln!(stdin, "cat /proc/{}/cmdline && echo \0", pid).ok()?;
        stdin.flush().ok()?;
        let mut buffer = Vec::new();
        stdout.read_until(b'\0', &mut buffer).ok()?;

        let output = String::from_utf8_lossy(&buffer);
        let package = output.trim_matches('\0').trim();

        let package = if package.is_empty() {
            format!("pid-{}", pid)
        } else {
            package.to_string()
        };
        cache.insert(pid.to_string(), package.clone());
        Some(package)
    }

    fn parse_line(
        raw_line: &str,
        cache: &mut HashMap<String, String>,
        stdin: &mut std::process::ChildStdin,
        stdout: &mut BufReader<std::process::ChildStdout>,
    ) -> Option<LogEntry> {
        let trimmed = raw_line.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 6 {
            return None;
        }

        let tag_and_message = parts[5..].join(" ");
        let (tag, message) = if let Some(colon_pos) = tag_and_message.find(':') {
            let tag = tag_and_message[..colon_pos].to_string();
            let message = tag_and_message[colon_pos + 1..].trim().to_string();
            (tag, message)
        } else {
            (tag_and_message, String::new())
        };

        Some(LogEntry {
            date: parts[0].to_owned(),
            time: parts[1].to_owned(),
            pid: parts[2].parse().ok(),
            tid: parts[3].parse().ok(),
            level: parts[4].to_owned(),
            tag,
            package: Self::get_package(parts[2], cache, stdin, stdout),
            message,
        })
    }
}

impl Drop for LogReader {
    fn drop(&mut self) {
        if let Some(mut logcat) = self.logcat.take() {
            let _ = logcat.kill();
        }
        if let Some(mut shell) = self.shell.take() {
            let _ = shell.kill();
        }
    }
}
