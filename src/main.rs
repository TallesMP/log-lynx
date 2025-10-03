use crossterm::style::{Color, ResetColor, SetForegroundColor};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn get_level_color(level: &str) -> Color {
    match level {
        "E" => Color::Red,
        "W" => Color::Yellow,
        "I" => Color::Green,
        "D" => Color::Cyan,
        "V" => Color::White,
        _ => Color::White,
    }
}
fn format_part(part: &str, size: usize, color: Option<Color>) -> String {
    let padded = if part.len() > size {
        part[..size].to_string()
    } else {
        format!("{:<width$}", part, width = size)
    };
    match color {
        Some(c) => format!("{}{}{}", SetForegroundColor(c), padded, ResetColor),
        None => padded,
    }
}

fn format_line(parts: Vec<&str>) -> String {
    if parts.len() < 6 {
        return String::new();
    }

    let date = parts[0];
    let hour = parts[1];
    let pid = parts[2];
    let tid = parts[3];
    let level = parts[4];
    let tag = parts[5];
    let message = parts[6..].join(" ");

    let level_color = get_level_color(level);

    let formatted_date = format_part(date, 10, None);
    let formatted_hour = format_part(hour, 8, None);
    let formatted_pid = format_part(pid, 5, None);
    let formatted_tid = format_part(tid, 5, None);
    let formatted_tag = format_part(tag, 15, None);
    let formatted_level = format_part(level, 1, Some(level_color));
    let formatted_message = format_part(&message, 150, Some(level_color));

    format!(
        "{} {} [{}-{}] {} {} {}",
        formatted_date,
        formatted_hour,
        formatted_pid,
        formatted_tid,
        formatted_tag,
        formatted_level,
        formatted_message
    )
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
