use std::{fs::read_to_string, io::prelude::*};

use anyhow::{Context, Result};
use chrono::Datelike;
use iridescent::{Styled, GREEN};

use crate::utils::{create_file, get_template_path, question};

/// Stateful representation of a new project environment. Primarily used to pass around all the
/// user-defined values to the various functions without passing tons of arguments.
#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub languages: Vec<String>,
    pub author: String,
    pub licenses: Vec<String>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            name: String::new(),
            languages: Vec::new(),
            licenses: Vec::new(),
            author: String::new(),
        }
    }

    /// Starts the new project creation process in the terminal.
    pub fn start(&mut self) -> Result<()> {
        println!(
            "{}",
            "Creating new project files...".foreground(GREEN).bold()
        );

        self.name = question(format!("{}", "Project Name?".bold()).as_str(), "")?;

        // Ask the user to specify the language(s) for the project.
        let mut language_text = Vec::with_capacity(3);

        language_text.push(format!(
            "{} {}",
            "Language(s)".bold(),
            "(space delimited)?".italic()
        ));
        language_text.push(format!(
            "{}: {}",
            "  Options".dim(),
            "rust python typescript"
        ));
        language_text.push(format!("{}: {}", "  Default".dim(), "rust"));

        self.languages = question(&language_text.join("\n"), "rust")?
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        self.author = question(format!("{}", "Author?".bold()).as_str(), "")?;

        // Ask the user to specify the license(s) for the project.
        let mut license_text = Vec::with_capacity(3);

        license_text.push(format!(
            "{} {}",
            "Licenses".bold(),
            "(space delimited)?".italic()
        ));
        license_text.push(format!("{}: {}", "  Options".dim(), "mit apache"));
        license_text.push(format!("{}: {}", "  Default".dim(), "mit"));

        self.licenses = question(&license_text.join("\n"), "mit")?
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // We want licenses to be alphabetically sorted to match template naming.
        self.licenses.sort();

        for lang in &self.languages {
            match lang.as_str() {
                "rust" | "rs" => {
                    self.generate_rust_specific_files()?;
                }
                "typescript" | "ts" => {}
                "python" | "py" => {}
                _ => {
                    println!("Unsupported language: {}", lang);
                }
            }
        }

        self.generate_licenses()?;
        self.generate_readme()?;
        self.generate_gitignore()?;
        self.generate_markdownlintignore()?;
        self.generate_changelog()?;
        self.generate_gitattributes()?;

        println!(
            "{}",
            "Project files created successfully!"
                .foreground(GREEN)
                .bold()
        );

        Ok(())
    }

    /// Replacing template placeholder values with user-defined values.
    fn search_and_replace(&self, s: String) -> String {
        let current_year = chrono::Local::now().year();

        s.replace("$$PROJECT_NAME", &self.name)
            .replace("$$CURRENT_YEAR", &current_year.to_string())
            .replace("$$PROJECT_AUTHOR", &self.author)
    }

    /// Creates a specific license string from a specified template.
    fn create_license(&self, license: &str) -> Result<String> {
        let template_path = get_template_path(license)
            .with_context(|| format!("Template `{}.txt` does not exist.", license))?;

        let template_file = read_to_string(template_path)?;
        let formatted_file = self.search_and_replace(template_file);

        Ok(formatted_file)
    }

    /// Generates all of the specified license files for the project.
    fn generate_licenses(&self) -> Result<()> {
        if self.licenses.len() > 1 {
            std::fs::create_dir("docs").context("`docs` directory already exists")?;

            let formatted_file =
                self.create_license(&format!("{}{}", &self.licenses.join("_"), "_license"))?;

            let mut output = create_file("LICENSE")?;

            output.write_all(formatted_file.as_bytes())?;

            for license in &self.licenses {
                match license.as_str() {
                    "mit" => {
                        let formatted_file = self.create_license("mit")?;
                        let mut output = create_file("docs/LICENSE-MIT")?;

                        output.write_all(formatted_file.as_bytes())?;
                    }
                    "apache" => {
                        let formatted_file = self.create_license("apache")?;
                        let mut output = create_file("docs/LICENSE-APACHE")?;

                        output.write_all(formatted_file.as_bytes())?;
                    }
                    _ => {
                        println!("Unsupported license: {}", license);
                    }
                }
            }
        } else {
            let formatted_file = self.create_license(&self.licenses[0])?;
            let mut output = create_file("LICENSE")?;

            output.write_all(formatted_file.as_bytes())?;
        }

        Ok(())
    }

    /// Generates a README.md file for the project.
    fn generate_readme(&self) -> Result<()> {
        let readme_template = if self.licenses.len() > 1 {
            format!("{}{}", &self.licenses.join("_"), "_readme")
        } else {
            "readme".to_string()
        };

        let template_path = get_template_path(&readme_template)
            .with_context(|| format!("Template `{}.txt` does not exist.", &readme_template))?;

        let template_file = read_to_string(template_path)?;
        let formatted_file = self.search_and_replace(template_file);
        let mut output = create_file("README.md")?;

        output.write_all(formatted_file.as_bytes())?;

        Ok(())
    }

    /// Generates a CHANGELOG.md file for the project.
    fn generate_changelog(&self) -> Result<()> {
        let template_path = get_template_path("changelog")
            .with_context(|| format!("Template `{}.txt` does not exist.", "changelog"))?;

        let template_file = read_to_string(template_path)?;
        let mut output = create_file("CHANGELOG.md")?;

        output.write_all(template_file.as_bytes())?;

        Ok(())
    }

    /// Generates a .markdownlint ignore file for the project.
    fn generate_markdownlintignore(&self) -> Result<()> {
        let template_path = get_template_path("markdownlintignore")
            .with_context(|| format!("Template `{}.txt` does not exist.", "markdownlintignore"))?;

        let template_file = read_to_string(template_path)?;

        let formatted_file = self.search_and_replace(template_file);

        let mut output = create_file(".markdownlintignore")?;

        output.write_all(formatted_file.as_bytes())?;

        Ok(())
    }

    /// Generates a .gitignore file for the project.
    fn generate_gitignore(&self) -> Result<()> {
        let template_path = get_template_path("gitignore")
            .with_context(|| format!("Template `{}.txt` does not exist.", "gitignore"))?;

        let template_file = read_to_string(template_path)?;
        let mut output = create_file(".gitignore")?;

        output.write_all(template_file.as_bytes())?;

        Ok(())
    }

    /// Generates a .gitattributes file for the project.
    fn generate_gitattributes(&self) -> Result<()> {
        let template_path = get_template_path("gitattributes")
            .with_context(|| format!("Template `{}.txt` does not exist.", "gitattributes"))?;

        let template_file = read_to_string(template_path)?;
        let mut output = create_file(".gitattributes")?;

        output.write_all(template_file.as_bytes())?;

        Ok(())
    }

    /// Generates Rust specific files for the project: deny.toml (for cargo deny) and
    /// rustfmt.toml for specific cargo fmt settings.
    fn generate_rust_specific_files(&self) -> Result<()> {
        let cargo_deny = get_template_path("deny")
            .with_context(|| format!("Template `{}.txt` does not exist.", "deny"))?;

        let rust_fmt = get_template_path("rustfmt")
            .with_context(|| format!("Template `{}.txt` does not exist.", "rustfmt"))?;

        let mut cargo_deny_file = create_file("deny.toml")?;
        let mut rust_fmt_file = create_file("rustfmt.toml")?;

        cargo_deny_file.write_all(read_to_string(cargo_deny)?.as_bytes())?;
        rust_fmt_file.write_all(read_to_string(rust_fmt)?.as_bytes())?;

        Ok(())
    }
}
