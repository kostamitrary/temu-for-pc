use std::process::Command;
use anyhow::Result;

pub fn run_application(path: &str) -> Result<String> {
    let output = Command::new(path).output()?;
    let result = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(result)
}