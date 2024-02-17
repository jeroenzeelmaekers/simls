use crate::structs::devices::Devices;
use std::process::Command;
use serde_json::Result;

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
