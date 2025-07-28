mod cli;
mod git;
use serde::Deserialize;
use std::fs;
use anyhow::{Result, Context};
use clap::Parser;
use colored::Colorize;
use dialoguer::{MultiSelect, theme::ColorfulTheme, Confirm};

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

fn run_checklist_interactive(checklist: &[String]) -> anyhow::Result<Vec<usize>> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please confirm each item before committing:")
        .items(checklist)
        .interact()?;
    Ok(selections)
}

fn build_todo_footer(checklist: &[String], checked_indices: &[usize]) -> String {
    if checked_indices.len() == checklist.len() {
        return String::new();
    }
    let mut footer = String::from("\nTODO:\n");
    for (i, item) in checklist.iter().enumerate() {
        if !checked_indices.contains(&i) {
            footer.push_str(&format!("- [ ] {}\n", item));
        }
    }
    footer
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let config = read_dod_config()?;
    if config.checklist.is_empty() {
        println!("{}", "No checklist items defined.".yellow());
    }

    match cli.command {
        cli::Commands::Status => {
            println!("--- Checking Git status ---");
            let status = git::status()?;
            println!("{}", format!("Git Status:\n{}", status).green());
        }
        cli::Commands::Commit { r#type, scope, message, no_verify, issue} => {
            println!("--- Committing changes ---");
            let scope_part = scope.map_or("".to_string(), |s| format!("({})", s));
            let header = format!("{}{}: {}", r#type, scope_part, message);
            let mut commit_message = format!("{}", header);

            if !no_verify {
                let checked = run_checklist_interactive(&config.checklist)?;
                if checked.len() != config.checklist.len() {
                    if Confirm::with_theme(&ColorfulTheme::default())
                        .with_prompt("Warning: Not all DoD items were checked. Proceed by adding a 'TODO' list to the commit message? (Y/n)")
                        .interact()?
                    {
                        let todo_footer = build_todo_footer(&config.checklist, &checked);
                        commit_message.push_str(&todo_footer);
                    } else {
                        println!("Commit aborted.");
                        return Ok(());
                    }
                }
                if config.issue_reference_required.unwrap_or(false) && !issue {
                    println!("{}", "Issue reference is required for commits.".red());
                    return Err(anyhow::anyhow!("Issue reference required"));
                }
            }

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