use std::process::Command;

use crate::config;

pub fn run_command(cmd: &str) -> String {
    let output = Command::new("sh").arg("-c").arg(cmd).output().expect("failed to exectute process");

    return String::from_utf8(output.stdout).unwrap_or_else(|_| { return config::UNKNOWNSTR.to_owned() }).trim_end().to_owned();
}
