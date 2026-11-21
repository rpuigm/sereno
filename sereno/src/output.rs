use std::io::{self, Write};

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

pub fn print_text(text: &str) {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let _ = writeln!(stdout, "{text}");
}

pub fn print_json<T>(payload: &T)
where
    T: Serialize,
{
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let output = serde_json::json!({
        "success": true,
        "data": payload,
    });
    let _ = writeln!(
        stdout,
        "{}",
        serde_json::to_string(&output).unwrap_or_else(|_| "{}".to_owned())
    );
}

pub fn print_json_error(error: &anyhow::Error) {
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    let output = serde_json::json!({
        "success": false,
        "error": error.to_string(),
    });
    let _ = writeln!(
        stderr,
        "{}",
        serde_json::to_string(&output).unwrap_or_else(|_| "{}".to_owned())
    );
}
