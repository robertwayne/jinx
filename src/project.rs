use std::fs::read_to_string;

use anyhow::{Context, Result};
use chrono::Datelike;
use iridescent::{Styled, GREEN};

use crate::utils::{get_template_path, question, try_write};

/// Stateful representation of a new project environment. Primarily used to pass
/// around all the user-defined values to the various functions without passing
/// tons of arguments.
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
        eprintln!(
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
        language_text.push(format!("{}: {}", "  Default".dim(), "(none)".italic()));

        self.languages = question(&language_text.join("\n"), "")?
            .split_whitespace()
            .map(ToString::to_string)
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
            .map(ToString::to_string)
            .collect();

        // We want licenses to be alphabetically sorted to match template
        // naming.
        self.licenses.sort();

        for lang in &self.languages {
            match lang.as_str() {
                "rust" | "rs" => Self::generate_rust_specific_files()?,
                "typescript" | "ts" => {}
                "python" | "py" => {}
                _ => {}
            }
        }

        self.generate_licenses()?;
        self.generate_readme()?;

        Self::generate_static_file("gitignore", ".gitignore")?;
        Self::generate_static_file("markdownlintignore", ".markdownlintignore")?;
        Self::generate_static_file("changelog", "CHANGELOG.md")?;
        Self::generate_static_file("gitattributes", ".gitattributes")?;

        eprintln!(
            "{}",
            "Project files created successfully!"
                .foreground(GREEN)
                .bold()
        );

        Ok(())
    }

    /// Replacing template placeholder values with user-defined values.
    fn search_and_replace(&self, s: &str) -> String {
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
        let formatted_file = self.search_and_replace(&template_file);

        Ok(formatted_file)
    }

    /// Generates all of the specified license files for the project.
    fn generate_licenses(&self) -> Result<()> {
        if self.licenses.len() > 1 {
            std::fs::create_dir("docs").context("`docs` directory already exists")?;

            let formatted_file =
                self.create_license(&format!("{}{}", &self.licenses.join("_"), "_license"))?;

            try_write("LICENSE", &formatted_file)?;

            for license in &self.licenses {
                match license.as_str() {
                    "mit" => {
                        let formatted_file = self.create_license("mit")?;
                        try_write("docs/LICENSE-MIT", &formatted_file)?;
                    }
                    "apache" => {
                        let formatted_file = self.create_license("apache")?;
                        try_write("docs/LICENSE-APACHE", &formatted_file)?;
                    }
                    _ => {
                        eprintln!("Unsupported license: {}", license);
                    }
                }
            }
        } else {
            let formatted_file = self.create_license(&self.licenses[0])?;
            try_write("LICENSE", &formatted_file)?;
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
        let formatted_file = self.search_and_replace(&template_file);

        try_write("README.md", &formatted_file)?;

        Ok(())
    }

    /// Generates Rust specific files for the project: deny.toml (for cargo
    /// deny) and rustfmt.toml for specific cargo fmt settings.
    fn generate_rust_specific_files() -> Result<()> {
        Self::generate_static_file("cargo_deny", "deny.toml")?;
        Self::generate_static_file("rustfmt", "rustfmt.toml")?;

        Ok(())
    }

    /// Generates files from a static template (e.g. .gitignore,
    /// .markdownlintignore, etc.)
    fn generate_static_file(template_name: &str, output_name: &str) -> Result<()> {
        let template_path = get_template_path(template_name)
            .with_context(|| format!("Template `{}.txt` does not exist.", template_name))?;

        let template_file = read_to_string(template_path)?;

        try_write(output_name, &template_file)?;

        Ok(())
    }
}
