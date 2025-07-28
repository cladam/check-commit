mod cli;
mod git;
use serde::Deserialize;
use std::fs;
use anyhow::{Result, Context};
use clap::Parser;
use colored::Colorize;

#[derive(Debug, Deserialize)]
struct DodConfig {
    issue_reference_required: Option<bool>,
    checklist: Vec<String>,
}

fn read_dod_config() -> Result<DodConfig> {
    let content = fs::read_to_string(".dod.yml")
        .context("Failed to read .dod.yml")?;
    let config: DodConfig = serde_yaml::from_str(&content)
        .context("Failed to parse .dod.yml")?;
    Ok(config)
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let config = read_dod_config()?;
    if !config.checklist.is_empty() {
        println!("Checklist items:");
        for item in config.checklist {
            println!("- {}", item);
        }
    } else {
        println!("{}", "No checklist items defined.".yellow());
    }

    match cli.command {
        cli::Commands::Status => {
            println!("--- Checking Git status ---");
            let status = git::status()?;
            println!("{}", format!("Git Status:\n{}", status).green());
        }
        cli::Commands::Commit { r#type, scope, message} => {
            println!("--- Committing changes ---");
            if config.issue_reference_required.unwrap_or(false) {
                println!("{}", "Issue reference is required for commits.".red());
                return Err(anyhow::anyhow!("Issue reference required"));
            }
            let scope_part = scope.map_or("".to_string(), |s| format!("({})", s));
            let header = format!("{}{}: {}", r#type, scope_part, message);
            let commit_message = format!("{}", header);

            println!("{}", format!("Commit message will be:\n---\n{}\n---", commit_message).blue());
            // Stage changes first, before any other operations.
            git::add_all()?;
            git::pull_latest_with_rebase()?;
            git::commit(&commit_message)?;
            git::push()?;
            println!("{}", "Successfully committed and pushed changes.".green());
        }
    }
    Ok(())
}