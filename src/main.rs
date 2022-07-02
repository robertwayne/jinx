mod project;
mod utils;

use std::process::exit;

use anyhow::{bail, Result};
use pico_args::Arguments;

use crate::project::Project;

const HELP: &str = "\
Jinx - quickly populate new project repositories

Usage:
    jinx [<args...>]

Flags:
    -h, --help       Displays help information
    -v, --version    Displays version information
";

/// Arguments passed in when running `jinx`.
#[derive(Debug)]
struct Args {
    _template: String,
}

fn main() -> Result<()> {
    let args = if let Ok(v) = parse_args() {
        v
    } else {
        eprintln!("{}", HELP);
        exit(1);
    };

    let Args { _template: _ } = args;
    Project::new().start()?;

    Ok(())
}

/// Parse command line arguments (if any).
fn parse_args() -> Result<Args> {
    let mut pargs = Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        println!("{HELP}");
        exit(0);
    }

    if pargs.contains(["-v", "--version"]) {
        println!("Jinx v{}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let args = Args {
        _template: "".to_string(),
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        bail!("Unknown arguments: {:?}", remaining);
    }

    Ok(args)
}
