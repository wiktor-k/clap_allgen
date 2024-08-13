// SPDX-FileCopyrightText: 2021-2023 Heiko Schaefer <heiko@schaefer.name>
// SPDX-FileCopyrightText: 2022 Lars Wirzenius <liw@liw.fi>
// SPDX-FileCopyrightText: 2022 Nora Widdecke <mail@nora.pink>
// SPDX-FileCopyrightText: 2023 David Runge <dave@sleepmap.de>
// SPDX-FileCopyrightText: 2024 Jan Christian Grünhage <jan.christian@gruenhage.xyz>
// SPDX-FileCopyrightText: 2024 Wiktor Kwapisiewicz <wiktor@metacode.biz>
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc = include_str!("../README.md")]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use carapace_spec_clap::Spec;
use clap::CommandFactory;
use clap::ValueEnum;
use clap_complete::generate_to;
use clap_complete::Shell;
use clap_complete_fig::Fig;
use clap_complete_nushell::Nushell;
use clap_mangen::Man;

/// Indicates an error during generation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The directory could not be created.
    #[error("Failed to create directory: {0}")]
    DirectoryCreate(#[source] std::io::Error),

    /// The shell completion file could not be created.
    #[error("Failed to create shell file {1}: {0}")]
    ShellFile(#[source] std::io::Error, String),

    /// The manpage could not be processed.
    #[error("Failed to process man file {1}: {0}")]
    ManFile(#[source] std::io::Error, PathBuf),
}

/// Render shell completion files to an output directory.
///
/// # Examples
///
/// The following example renders shell completions for all supported
/// shells to the `/tmp/shell-completions` directory.
///
/// ```
/// # fn main() -> testresult::TestResult {
/// use clap_allgen::render_shell_completions;
///
/// #[derive(Debug, clap::Parser)]
/// enum Commands {
///     First,
///     Second,
///     Third,
/// }
///
/// render_shell_completions::<Commands>("/tmp/shell-completions")?;
/// # Ok(()) }
/// ```
pub fn render_shell_completions<T: CommandFactory>(
    output_dir: impl AsRef<Path>,
) -> Result<(), Error> {
    let output_dir = output_dir.as_ref();

    let mut command = T::command();

    create_dir_all(output_dir).map_err(Error::DirectoryCreate)?;

    let bin_name = command
        .get_bin_name()
        .unwrap_or(command.get_name())
        .to_string();

    for shell in Shell::value_variants() {
        generate_to(*shell, &mut command, &bin_name, output_dir)
            .map_err(|e| Error::ShellFile(e, shell.to_string()))?;
    }

    generate_to(Nushell, &mut command, &bin_name, output_dir)
        .map_err(|e| Error::ShellFile(e, "nushell".to_string()))?;

    generate_to(Fig, &mut command, &bin_name, output_dir)
        .map_err(|e| Error::ShellFile(e, "fig".to_string()))?;

    generate_to(Spec, &mut command, &bin_name, output_dir)
        .map_err(|e| Error::ShellFile(e, "carapace".to_string()))?;

    Ok(())
}

/// Render man pages to an output directory.
///
/// # Examples
///
/// The following example renders manpages to the `/tmp/man-pages`
/// directory.
///
/// ```
/// # fn main() -> testresult::TestResult {
/// use clap_allgen::render_manpages;
///
/// #[derive(Debug, clap::Parser)]
/// enum Commands {
///     First,
///     Second,
///     Third,
/// }
///
/// render_manpages::<Commands>("/tmp/man-pages")?;
/// # Ok(()) }
/// ```
pub fn render_manpages<T: CommandFactory>(output_dir: impl AsRef<Path>) -> Result<(), Error> {
    let output_dir = output_dir.as_ref();

    let mut command = T::command();

    /// Render man pages for commands and subcommands recursively
    fn render_recursive(
        output_dir: &Path,
        command: &mut clap::Command,
        prefix: &str,
    ) -> Result<(), Error> {
        // prefix name with name of parent command if we are a subcommand
        // NOTE: this is not ideal yet, as we are getting e.g. `command-subcommand` instead of
        // `command subcommand` in SYNOPSIS, however this is due to a clap_mangen limitation:
        // https://github.com/clap-rs/clap/discussions/3603
        let name = if !prefix.is_empty() {
            format!("{}-{}", prefix, command.get_name())
        } else {
            command.get_name().to_string()
        };

        let command = &mut command.clone().name(&name);

        let file_name = output_dir.join(format!("{name}.1"));

        let mut out = File::create(&file_name).map_err(|e| Error::ManFile(e, file_name.clone()))?;
        Man::new(command.clone())
            .render(&mut out)
            .map_err(|e| Error::ManFile(e, file_name.clone()))?;
        out.flush()
            .map_err(|e| Error::ManFile(e, file_name.clone()))?;

        // get the current command's name to prefix any further subcommands
        let cmd_name = command.get_name().to_string();
        for subcommand in command.get_subcommands_mut() {
            // we do not want man pages for the help subcommands
            if !subcommand.get_name().contains("help") {
                render_recursive(output_dir, subcommand, &cmd_name)?;
            }
        }

        Ok(())
    }

    create_dir_all(output_dir).map_err(Error::DirectoryCreate)?;

    command.build();

    render_recursive(output_dir, &mut command, "")?;

    Ok(())
}
