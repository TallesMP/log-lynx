use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn format_line(parts: Vec<&str>) -> String {
    let (date, hour, pid, tid, level, tag) = (
        parts[0],
        parts[1],
        parts[2],
        parts[3],
        parts[4].to_string(),
        parts[5],
    );
    let message = parts[6..].join(" ");

    let formatted_pid = set_size(pid, 5);
    let formatted_tid = set_size(tid, 5);
    let formatted_tag = set_size(tag, 15);
    let formatted_level = set_size(&level, 1);
    let formatted_message = set_size(&message, 150);
    let result = format!(
        "{} {} [{}-{}] {} {} {}",
        date, hour, formatted_pid, formatted_tid, formatted_tag, formatted_level, formatted_message,
    );
    return result;
}

fn set_size(part: &str, size: usize) -> String {
    if part.len() > size {
        return format!("{}", &part[..size]);
    }
    return format!("{:<padding$}", part, padding = size);
}

fn main() {
    let mut child = Command::new("adb")
        .arg("logcat")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start adb process");

    let stdout = child.stdout.take().expect("stdout capture error");
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        match line {
            Ok(line_string) => {
                let parts: Vec<&str> = line_string.split_whitespace().collect();
                if parts.len() < 6 {
                    continue;
                }
                let formatted_line = format_line(parts);

                println!("{}", formatted_line);
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}
