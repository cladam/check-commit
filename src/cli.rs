use anyhow::{Context, Result};
use clap::{Command, CommandFactory, Parser, Subcommand};
use colored::*;
use std::io::Write;
use thiserror::Error;

/// A CLI to streamline your Git workflow for Trunk-Based Development
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Show the current Git status
    Status,
    /// Commits changes to the current branch or 'main' if no branch is checked out.
    #[command(after_help = "Use the imperative, present tense: \"change\" not \"changed\". Think of This commit will...\n\
    COMMON COMMIT TYPES:\n  \
    feat:     A new feature for the user.\n  \
    fix:      A bug fix for the user.\n  \
    chore:    Routine tasks, maintenance, dependency updates.\n  \
    docs:     Documentation changes.\n  \
    style:    Code style changes (formatting, etc).\n  \
    refactor: Code changes that neither fix a bug nor add a feature.\n  \
    test:     Adding or improving tests.\n\n\
    EXAMPLES:\n  \
    check-commit commit --type \"feat\" --scope api -m \"Add user endpoint\"")]
    Commit {
        /// Commit type (e.g. 'feat', 'fix', 'chore', 'docs').
        #[arg(short, long)]
        r#type: String,
        /// Optional scope of the commit.
        #[arg(short, long)]
        scope: Option<String>,
        /// The descriptive commit message.
        #[arg(short, long)]
        message: String,
    },
}