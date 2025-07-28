use std::process::{Command, Stdio};
use thiserror::Error;
use anyhow::{Context, Result};

// Using `thiserror` to create a structured error type.
#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    Git(String),
}

/// Runs a Git command with the specified subcommand and arguments.
fn run_git_command(command: &str, args: &[&str]) -> Result<String> {
    println!("[RUNNING] git {} {}", command, args.join(" "));
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
pub fn status() -> Result<String> {
    run_git_command("status", &["--short"])
}

/// Pull the latest changes with rebase.
pub fn pull_latest_with_rebase() -> Result<String> {
    // Using --autostash to safely handle local changes before pulling.
    run_git_command("pull", &["--rebase", "--autostash"])
}

/// Add all changes to the staging area.
pub fn add_all() -> Result<String> {
    run_git_command("add", &["."])
}

/// Commit changes with a message.
pub fn commit(message: &str) -> Result<String> {
    run_git_command("commit", &["-m", message])
}

/// Push changes to the remote repository.
pub fn push() -> Result<String> {
    run_git_command("push", &[])
}
