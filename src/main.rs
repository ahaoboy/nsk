use std::error::Error as _;
use std::process::ExitCode;

use clap::Parser;
use nsk::Cli;

fn main() -> ExitCode {
    match nsk::cli::run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            let mut source = err.source();
            while let Some(cause) = source {
                eprintln!("  caused by: {cause}");
                source = cause.source();
            }
            ExitCode::FAILURE
        }
    }
}
