use std::process::{Command, Stdio};
use thiserror::Error;
use anyhow::{Context, Result};
use colored::Colorize;
use crate::git;

// Using `thiserror` to create a structured error type.
#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    Git(String),
    #[error("Working directory is not clean: {0}")]
    DirectoryNotClean(String),
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

/// Checks if the git working directory is clean.
pub fn is_working_directory_clean() -> Result<()> {
    let output = run_git_command("status", &["--porcelain"])?;
    if output.is_empty() {
        Ok(())
    } else {
        Err(GitError::DirectoryNotClean(
            "You have unstaged changes. Please commit or stash them first.".to_string()
        ).into())
    }
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
