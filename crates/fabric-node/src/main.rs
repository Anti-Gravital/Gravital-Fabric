//! Local-only prototype node agent for Gravital Fabric.
//!
//! The binary discovers host facts, derives a stable local identifier and emits
//! JSON. It never opens a socket or contacts an external service. It is a
//! prototype slice of Phase 1, not an installable Fabric node agent.

mod discovery;
mod identity;
mod manifest;
mod register;

use std::fmt;
use std::path::PathBuf;
use std::process::ExitCode;

const VERSION: &str = "0.1.0";
const DEFAULT_STATE_FILE: &str = "fabric-node-state.json";

enum CliError {
    Usage(String),
    Io(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Usage(message) | CliError::Io(message) => write!(formatter, "{message}"),
        }
    }
}

fn usage() {
    println!("fabric-node {VERSION} (local-only prototype)");
    println!();
    println!("Usage: fabric-node <COMMAND> [OPTIONS]");
    println!();
    println!("Commands:");
    println!("  manifest                 Print the local node manifest as JSON");
    println!(
        "  register [--state PATH]  Write manifest and registration time to a local state file"
    );
    println!("  --version                Print the version");
    println!("  --help                   Print this help");
    println!();
    println!("Default state path: ./{DEFAULT_STATE_FILE}");
}

fn run() -> Result<(), CliError> {
    let mut args = std::env::args_os().skip(1);
    let Some(command) = args.next() else {
        usage();
        return Ok(());
    };
    let command = command.to_string_lossy();
    match command.as_ref() {
        "manifest" => {
            if args.next().is_some() {
                return Err(CliError::Usage("manifest takes no arguments".to_string()));
            }
            let found = manifest::collect().map_err(CliError::Io)?;
            println!("{}", manifest::to_json(&found));
            Ok(())
        }
        "register" => {
            let mut state_path = PathBuf::from(DEFAULT_STATE_FILE);
            while let Some(argument) = args.next() {
                let argument = argument.to_string_lossy();
                match argument.as_ref() {
                    "--state" => {
                        let value = args
                            .next()
                            .ok_or_else(|| CliError::Usage("--state needs a path".to_string()))?;
                        if value.is_empty() {
                            return Err(CliError::Usage(
                                "--state needs a non-empty path".to_string(),
                            ));
                        }
                        state_path = PathBuf::from(value);
                    }
                    other => {
                        return Err(CliError::Usage(format!("unknown register option: {other}")));
                    }
                }
            }
            let found = manifest::collect().map_err(CliError::Io)?;
            let registered_at = register::write_state(&state_path, &found).map_err(CliError::Io)?;
            println!(
                "registered {} at {} (unix {registered_at})",
                found.node.node_id,
                state_path.display()
            );
            Ok(())
        }
        "--version" | "-V" => {
            println!("fabric-node {VERSION}");
            Ok(())
        }
        "--help" | "-h" => {
            usage();
            Ok(())
        }
        other => Err(CliError::Usage(format!("unknown command: {other}"))),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("fabric-node: {error}");
            match error {
                CliError::Usage(_) => ExitCode::from(2),
                CliError::Io(_) => ExitCode::FAILURE,
            }
        }
    }
}
