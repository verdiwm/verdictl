use anyhow::{bail, Result};
use sap::{Argument, Parser};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const VERDI_VERSION: &str = "0.1.0";

const HELP: &str = "\
verdictl - CLI to interact with the Verdi compositor

USAGE:
    verdictl [OPTIONS] <COMMAND>

OPTIONS:
    --help       Print this help message
    --version    Print version information

COMMANDS:
    version      Print the Verdi compositor version";

fn help() {
    println!("{HELP}");
}

fn cmd_version() -> Result<()> {
    println!("Verdi version: {VERDI_VERSION}");
    Ok(())
}

fn main() -> Result<()> {
    let mut parser = Parser::from_env()?;

    // Check for top-level flags or a subcommand
    match parser.forward()? {
        None => {
            help();
        }
        Some(Argument::Long("help")) => {
            help();
        }
        Some(Argument::Long("version")) => {
            println!("verdictl {VERSION}");
        }
        Some(Argument::Value(cmd)) => match cmd.as_ref() {
            "version" => cmd_version()?,
            other => bail!("unsupported command: {other}"),
        },
        Some(arg) => {
            let err: sap::ParsingError = arg.unexpected();
            bail!("unsupported command: {err}");
        }
    }

    Ok(())
}
