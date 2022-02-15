# Jinx

Jinx is a CLI tool for populating fresh repositories with standard files, such as a README, licenses, and
language-specific settings files.

*This currently has very minimal support for project environments, namely supporting the types of
projects I build myself regularly. I do intend on adding more template support and deeper argument
handling in the future.*

## Features

Supports Rust, TypeScript (Node), and Python project structures.

Generates: a skeleton README.md, a generic .gitignore, a blank CHANGELOG.md, a .markdownlintignore, and one or more LICENSE files.

For Rust projects, additionally generates a deny.toml *(for `cargo deny`)* and a rustfmt.toml file *(reorder & squish imports)*.

## Usage

Simply run `jinx` in the directory you wish to populate with project files.

Currently does not support supplying a path as an argument.

## Installation

<!-- markdownlint-disable -->

<details>
  <summary>Linux</summary>

  1. Download the [latest release](https://github.com/robertwayne/jinx/releases).
  2. Extract the files with `tar --xz -xf jinx.tar.xz --directory <wherever you want>`.
  3. Move to the new directory and grant executable permissions to the binary with `sudo chmod +x
     jinx`.
  4. Add the directory location to your PATH so you can run it from anywhere (eg. `export
     PATH="$PATH/bin:$PATH"`).

  You're all set! Run `jinx` in a new directory or `jinx --help` to see the help file.
</details>

<details>
  <summary>Windows</summary>

  1. Download the [latest release](https://github.com/robertwayne/jinx/releases).
  2. Extract the files with [7zip](https://www.7-zip.org/) to wherever you want.
  3. Add the directory location to your PATH like so:
     1. In your search bar, type `environment variables`.
     2. Select `Edit the system environment variables`.
     3. Click `Environment Variables` near the bottom.
     4. Double-click `Path` in the `User variables for x` box.
     5. Click `New`.
     6. Type in the directory you extracted `jinx` and `templates` to.
     7. Click `Ok`.

  You're all set! Run `jinx` in a new directory or `jinx --help` to see the help file.
</details>

<!-- markdownlint-enable -->

## License

Jinx source code is dual-licensed under either

- **[MIT License](/docs/LICENSE-MIT.md)**
- **[Apache License, Version 2.0](/docs/LICENSE-APACHE.md)**

at your option.
