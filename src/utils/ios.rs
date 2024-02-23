use crate::structs::ios_devices::Devices;
use serde_json::Result;
use std::process::Command;

pub fn read_ios_devices() -> Result<Devices> {
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

pub fn start_ios_simulator(udid: &str) {
    if cfg!(target_os = "macos") {
        Command::new("xcrun")
            .args(["simctl", "boot", udid])
            .output()
            .expect("Failed to boot simulator");
        Command::new("open")
            .args(["-a", "Simulator"])
            .output()
            .expect("Failed to open Simulator.app");
        Command::new("osascript")
            .args([
                "-e",
                r#"if running of application "Simulator" then
          tell application "System Events"
            set theWindows to windows of (processes whose name is "Simulator")
            repeat with theWindow in (the first item of theWindows)
              set theWindowName to name of theWindow
              if theWindowName contains "${device.name}" then
                perform action "AXRaise" of theWindow
              end if
            end repeat
          end tell
          tell the application "Simulator"
            activate
          end tell
        end if"#,
            ])
            .output()
            .expect("Failed to open Simulator.app");
    } else {
        Command::new("echo")
            .args(["No support for your OS yet"])
            .output()
            .expect("Failed to execute command");
    }
}

pub fn stop_ios_simulator(udid: &str) {
    if cfg!(target_os = "macos") {
        Command::new("xcrun")
            .args(["simctl", "shutdown", udid])
            .output()
            .expect("Failed to shutdown simulator");
    } else {
        Command::new("echo")
            .args(["No support for your OS yet"])
            .output()
            .expect("Failed to execute command");
    }
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
