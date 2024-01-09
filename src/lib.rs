// SPDX-FileCopyrightText: 2021-2023 Heiko Schaefer <heiko@schaefer.name>
// SPDX-FileCopyrightText: 2022 Lars Wirzenius <liw@liw.fi>
// SPDX-FileCopyrightText: 2022 Nora Widdecke <mail@nora.pink>
// SPDX-FileCopyrightText: 2023 David Runge <dave@sleepmap.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::MAIN_SEPARATOR;

use anyhow::anyhow;
use anyhow::Result;
use clap::CommandFactory;

use clap_complete::generate_to;
use clap_complete::Shell;
use clap_mangen::Man;

/// Render shell completion files to an output directory
pub fn render_shell_completions<T: CommandFactory>(
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = T::command();

    create_dir_all(output_dir)
        .map_err(|_| anyhow!("Failed to create directory: {}", output_dir.display()))?;

    println!(
        "Writing shell completions to {}",
        output_dir.to_str().unwrap_or(&format!("{:?}", output_dir))
    );
    let bin_name = command
        .get_bin_name()
        .unwrap_or(command.get_name())
        .to_string();

    for shell in &[
        Shell::Bash,
        Shell::Elvish,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Zsh,
    ] {
        generate_to(*shell, &mut command, &bin_name, output_dir).map_err(|_| {
            anyhow!(
                "Failed to create file: {}{}{}",
                output_dir.display(),
                MAIN_SEPARATOR,
                shell
            )
        })?;
    }
    Ok(())
}

/// Render man pages to an output directory
pub fn render_manpages<T: CommandFactory>(
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = T::command();

    /// Render man pages for commands and subcommands recursively
    fn render_recursive(
        output_dir: &Path,
        command: &mut clap::Command,
        prefix: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

        let mut out = File::create(output_dir.join(format!("{name}.1"))).map_err(|_| {
            anyhow!(
                "Failed creating {}",
                output_dir.join(format!("{name}.1")).display()
            )
        })?;
        Man::new(command.clone()).render(&mut out).map_err(|_| {
            anyhow!(
                "Failed rendering {}",
                output_dir.join(format!("{name}.1")).display()
            )
        })?;
        out.flush().map_err(|_| {
            anyhow!(
                "Failed writing {}",
                output_dir.join(format!("{name}.1")).display()
            )
        })?;

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

    create_dir_all(output_dir)
        .map_err(|_| anyhow!("Failed creating directory {}", output_dir.display()))?;

    command.build();

    println!(
        "Writing man pages to {}",
        output_dir.to_str().unwrap_or(&format!("{:?}", output_dir))
    );

    render_recursive(output_dir, &mut command, "")?;

    Ok(())
}
