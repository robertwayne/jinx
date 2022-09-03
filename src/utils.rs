use std::{
    fs::{File, OpenOptions},
    io::{stdin, Write},
};

use anyhow::{Context, Result};
use iridescent::Styled;

/// Gets the path to the template directory. When building in release, this will
/// set the path to the template directory to the same directory the executable
/// file is in - NOT where the command is run from. Whenever a build is created
/// without debug symbols, the template directory is wherever the Cargo.toml
/// file is located.
pub fn get_template_path(file: &str) -> Result<String> {
    #[cfg(not(debug_assertions))]
    let executable_path = std::env::current_exe()?;

    #[cfg(not(debug_assertions))]
    let data = executable_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Could not get parent directory"))?
        .join(format!(".jinx-templates/{file}.txt"))
        .display()
        .to_string();

    #[cfg(debug_assertions)]
    let executable_path = std::env::current_dir()?;

    #[cfg(debug_assertions)]
    let data = executable_path.join(format!(".jinx-templates/{file}.txt")).display().to_string();

    Ok(data)
}

/// Creates a new file with the specified name.
fn create_file(file: &str) -> Result<File, anyhow::Error> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file)
        .context(format!("Could not open `{file}`"))
}

/// Attempts to write a new file with the provided name. If the file already
/// exists, it will skip writing that file and continue.
pub fn try_write(name: &str, data: &str) -> Result<()> {
    if let Ok(mut output) = create_file(name) {
        output.write_all(data.as_bytes())?;
    } else {
        eprintln!("Skipping {} because it already exists.", name.bold());
    }

    Ok(())
}

/// Takes a question and a default value, displays the question, and attempts to
/// parse user input to return the relevant value. Will apply the default value
/// when the user line input is empty.
pub fn question(display: &str, default: &str) -> Result<String> {
    println!("{}", display);
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    if input.trim().is_empty() {
        return Ok(default.into());
    }

    Ok(input.trim().to_string())
}
