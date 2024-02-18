use serde_json::Result;
use std::process::Command;

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
