use crossterm::style::{Color, ResetColor, SetForegroundColor};
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

fn get_level_color(level: &str) -> Color {
    match level {
        "E" => Color::Red,
        "W" => Color::Yellow,
        "I" => Color::Green,
        "D" => Color::Cyan,
        "V" | _ => Color::White,
    }
}

fn set_color(text: &str, color: Option<Color>) -> String {
    color.map_or_else(
        || text.to_string(),
        |c| format!("{}{}{}", SetForegroundColor(c), text, ResetColor),
    )
}

fn pad(text: &str, width: usize) -> String {
    if text.len() > width {
        text[..width].to_string()
    } else {
        format!("{:<width$}", text, width = width)
    }
}

fn get_package(pid: &str, stdin: &mut impl Write, reader: &mut BufReader<impl io::Read>) -> String {
    let start = Instant::now();

    writeln!(stdin, "cat /proc/{}/cmdline", pid).expect("Failed to write to shell");
    stdin.flush().expect("Failed to flush stdin");

    let mut package = String::new();
    let mut line = String::new();
    if reader.read_line(&mut line).is_ok() {
        package = line.trim_matches('\0').trim().to_string();
    }
    line.clear();

    let time = start.elapsed();
    println!("debug get_package time: {:?}", time);

    package
}

fn format_line(
    parts: &[&str],
    stdin: &mut impl Write,
    reader: &mut BufReader<impl io::Read>,
    cache: &mut HashMap<String, String>,
) -> String {
    if parts.len() < 7 {
        return String::new();
    }

    let date = pad(parts[0], 5);
    let time = pad(parts[1], 8);
    let pid = pad(parts[2], 5);
    let tid = pad(parts[3], 5);
    let level = parts[4];
    let tag = pad(parts[5], 15);

    let package = cache
        .entry(parts[2].to_string())
        .or_insert_with(|| get_package(parts[2], stdin, reader))
        .clone();
    let package = pad(&package, 15);

    let message = parts[6..].join(" ");
    let level_color = get_level_color(level);
    let level_colored = set_color(level, Some(level_color));
    let msg_colored = set_color(&message, Some(level_color));

    format!("{date} {time} [{pid}-{tid}] {tag} {package} {level_colored} {msg_colored}")
}

fn main() -> io::Result<()> {
    let mut child = Command::new("adb")
        .arg("shell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    let stdout = child.stdout.as_mut().expect("Failed to open stdout");
    let mut reader = BufReader::new(stdout);

    writeln!(stdin, "logcat")?;
    stdin.flush()?;

    let mut package_cache: HashMap<String, String> = HashMap::new();

    let mut line = String::new();
    while reader.read_line(&mut line).is_ok() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            line.clear();
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() >= 7 {
            println!(
                "{}",
                format_line(&parts, stdin, &mut reader, &mut package_cache)
            );
        }
        line.clear();
    }

    child.wait()?;
    Ok(())
}
