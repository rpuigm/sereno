use std::io;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use serde::Serialize;

use crate::output::{OutputFormat, print_json, print_text};

#[derive(Debug, Parser)]
#[command(
    name = "sereno",
    version,
    about = "A lightweight Rust PKI and security CLI"
)]
pub struct Cli {
    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Text, global = true)]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Command::Version => {
                let payload = VersionPayload {
                    name: env!("CARGO_PKG_NAME"),
                    version: env!("CARGO_PKG_VERSION"),
                };
                match self.format {
                    OutputFormat::Text => {
                        print_text(&format!("{} {}", payload.name, payload.version))
                    }
                    OutputFormat::Json => print_json(&payload),
                }
            }
            Command::Doctor => {
                let payload = DoctorPayload {
                    binary: env!("CARGO_PKG_NAME"),
                    version: env!("CARGO_PKG_VERSION"),
                    os: std::env::consts::OS,
                    arch: std::env::consts::ARCH,
                    status: "ready",
                };
                match self.format {
                    OutputFormat::Text => print_text(&format!(
                        "{} {}: ready\nOS: {}\nArchitecture: {}",
                        payload.binary, payload.version, payload.os, payload.arch
                    )),
                    OutputFormat::Json => print_json(&payload),
                }
            }
            Command::Completion { shell } => {
                let mut command = Cli::command();
                generate(shell, &mut command, "sereno", &mut io::stdout());
            }
        }

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print the Sereno name and version
    Version,
    /// Check the basic local runtime environment
    Doctor,
    /// Generate shell completion scripts
    Completion {
        /// Shell for which to generate completion
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Serialize)]
struct VersionPayload {
    name: &'static str,
    version: &'static str,
}

#[derive(Debug, Serialize)]
struct DoctorPayload {
    binary: &'static str,
    version: &'static str,
    os: &'static str,
    arch: &'static str,
    status: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_global_format_before_subcommand() {
        let cli = Cli::try_parse_from(["sereno", "--format", "json", "version"]).unwrap();
        assert!(matches!(cli.format, OutputFormat::Json));
        assert!(matches!(cli.command, Command::Version));
    }

    #[test]
    fn parses_global_format_after_subcommand() {
        let cli = Cli::try_parse_from(["sereno", "doctor", "--format", "json"]).unwrap();
        assert!(matches!(cli.format, OutputFormat::Json));
        assert!(matches!(cli.command, Command::Doctor));
    }
}
