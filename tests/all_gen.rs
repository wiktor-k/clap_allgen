use clap::CommandFactory;
use clap::Parser;
use rstest::{fixture, rstest};
use std::path::PathBuf;

use clap_allgen::{render_manpages, render_shell_completions};
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
    render_manpages(&mut Commands::command(), &path)?;
    assert!(path.join("this-is-example.1").exists());

    Ok(())
}

#[rstest]
#[trace]
fn shell_completions(#[from(create_tempdir)] path: PathBuf) -> testresult::TestResult {
    render_shell_completions(&mut Commands::command(), "test", &path)?;
    assert!(path.join("test.bash").exists());
    Ok(())
}
