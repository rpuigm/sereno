mod cli;
mod output;

use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            let exit_code = error.exit_code();
            let _ = error.print();
            return ExitCode::from(exit_code as u8);
        }
    };

    let format = cli.format;
    match cli.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            if format == crate::output::OutputFormat::Json {
                output::print_json_error(&error);
            } else {
                eprintln!("Error: {error:#}");
            }
            ExitCode::FAILURE
        }
    }
}
