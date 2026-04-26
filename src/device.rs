use std::{
    io::{self, BufRead, BufReader},
    process::Command,
};

pub struct Device {
    pub serial: String,
    pub model: String,
    pub authorized: bool,
}

pub struct ConnectedDevices {
    pub devices: Vec<Device>,
}

impl ConnectedDevices {
    pub fn update() -> io::Result<Self> {
        let output = Command::new("adb")
            .args(["devices", "-l"])
            .output()?;

        let reader = BufReader::new(output.stdout.as_slice());

        let devices = reader
            .lines()
            .skip(1)
            .filter_map(|line| line.ok())
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 {
                    return None;
                }

                let serial = parts[0].to_string();
                let status = parts[1];
                let authorized = status == "device";

                let model = parts
                    .iter()
                    .find(|p| p.starts_with("model:"))
                    .map(|p| p[6..].to_string())
                    .unwrap_or_else(|| {
                        if !authorized {
                            "Unauthorized".to_string()
                        } else {
                            "Unknown".to_string()
                        }
                    });

                Some(Device {
                    serial,
                    model,
                    authorized,
                })
            })
            .collect();

        Ok(Self { devices })
    }
}
