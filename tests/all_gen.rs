use std::path::PathBuf;

use clap::Parser;
use clap_allgen::{render_manpages, render_shell_completions};
use rstest::{fixture, rstest};
/// This is a CLI tool
#[derive(Debug, Parser)]
#[command(name = "this-is-example")]
enum Commands {
    /// First command
    First,
    /// Second command
    Second,
    /// Third command
    Third,
}

#[fixture]
fn create_tempdir() -> PathBuf {
    tempfile::tempdir().expect("tempdir to work").into_path()
}

#[rstest]
#[trace]
fn manpages(#[from(create_tempdir)] path: PathBuf) -> testresult::TestResult {
    render_manpages::<Commands>(&path)?;
    assert!(path.join("this-is-example.1").exists());

    Ok(())
}

#[rstest]
#[trace]
fn shell_completions(#[from(create_tempdir)] path: PathBuf) -> testresult::TestResult {
    render_shell_completions::<Commands>(&path)?;
    assert!(path.join("this-is-example.bash").exists());
    Ok(())
}
