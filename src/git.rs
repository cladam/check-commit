use std::process::{Command, Stdio};
use thiserror::Error;
use anyhow::{Context, Result};
use colored::Colorize;

// Using `thiserror` to create a structured error type.
#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git command failed terribly: {0}")]
    Git(String),
}

/// Runs a Git command with the specified subcommand and arguments.
fn run_git_command(command: &str, args: &[&str], verbose: bool) -> Result<String> {
    if verbose {
        println!("{} git {} {}", "[RUNNING] ".cyan(), command, args.join(" "));
    }
    let output = Command::new("git")
        .arg(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("Failed to execute 'git {}'", command))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(GitError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()).into())
    }
}

/// Show the current status of the repository.
pub fn status(verbose: bool) -> Result<String> {
    run_git_command("status", &["--short"], verbose)
}

/// Pull the latest changes with rebase.
pub fn pull_latest_with_rebase(verbose: bool) -> Result<String> {
    // Using --autostash to safely handle local changes before pulling.
    run_git_command("pull", &["--rebase", "--autostash"], verbose)
}

/// Add all changes to the staging area.
pub fn add_all(verbose: bool) -> Result<String> {
    run_git_command("add", &["."], verbose)
}

/// Commit changes with a message.
pub fn commit(message: &str, verbose: bool) -> Result<String> {
    run_git_command("commit", &["-m", message], verbose)
}

/// Push changes to the remote repository.
pub fn push(verbose: bool) -> Result<String> {
    run_git_command("push", &[], verbose)
}
