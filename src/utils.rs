use crate::structs::devices::Devices;
use serde_json::Result;
use std::process::Command;

pub fn read_ios_simulators() -> Result<Devices> {
    let output = if cfg!(target_os = "macos") {
        Command::new("xcrun")
            .args(["simctl", "list", "--json", "devices"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("echo")
            .args(["No support for your OS yet"])
            .output()
            .expect("Failed to execute command")
    };

    let output_string = String::from_utf8(output.stdout).unwrap();

    let devices: Devices = serde_json::from_str(&output_string)?;

    Ok(devices)
}

pub fn extract_ios_version(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.split(&['.', '-'][..]).collect();

    for i in 0..parts.len() {
        if parts[i] == "iOS" && i + 2 < parts.len() {
            if let (Ok(major), Ok(minor)) =
                (parts[i + 1].parse::<u32>(), parts[i + 2].parse::<u32>())
            {
                return Some(format!("{}.{}", major, minor));
            }
        }
    }
    None
}

pub fn read_android_emulators() -> Result<Vec<String>> {
    let output = if cfg!(target_os = "macos") {
        Command::new("emulator")
            .args(["-list-avds"])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("echo")
            .args(["No support for your OS yet"])
            .output()
            .expect("Failed to execute command")
    };

    let output_string = String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    Ok(output_string)
}
