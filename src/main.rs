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

/// Reads the DoD configuration from `.dod.yml` file in the current directory (root of the git repository).
fn read_dod_config() -> Result<DodConfig> {
    let content = fs::read_to_string(".dod.yml")
        .context("Failed to read .dod.yml")?;
    let config: DodConfig = serde_yaml::from_str(&content)
        .context("Failed to parse .dod.yml")?;
    Ok(config)
}

/// Runs the checklist interactively, allowing the user to confirm each item before committing.
fn run_checklist_interactive(checklist: &[String]) -> anyhow::Result<Vec<usize>> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please confirm each item before committing:")
        .items(checklist)
        .interact()?;
    Ok(selections)
}

/// Builds the TODO footer for the commit message based on unchecked items in the checklist.
fn build_todo_footer(checklist: &[String], checked_indices: &[usize]) -> String {
    //let checked_indices: Vec<usize> = checked_indices.iter().cloned().collect();
    let unchecked_items: Vec<String> = checklist
        .iter()
        .enumerate()
        .filter(|(i, _)| !checked_indices.contains(&i))
        .map(|(_, item)| format!("- [ ] {}", item))
        //.filter_map(|(i, item)| if !checked_indices.contains(&i) { Some(item.clone()) } else { None })
        .collect();
    if unchecked_items.is_empty() {
        String::new()
    } else {
        format!("\n\nTODO:\n{}", unchecked_items.join("\n"))
    }
}

/// Handles the interactive commit process, including checklist confirmation and issue reference handling.
fn handle_interactive_commit(
    config: &DodConfig,
    base_message: &str,
    issue: &Option<String>,
) -> Result<Option<String>> {
    let mut commit_message = base_message.to_string();

    let checked = run_checklist_interactive(&config.checklist)?;
    if checked.len() != config.checklist.len() {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Warning: Not all DoD items were checked. Proceed by adding a 'TODO' list to the commit message?")
            .interact()?
        {
            let todo_footer = build_todo_footer(&config.checklist, &checked);
            commit_message.push_str(&todo_footer);
        } else {
            println!("Commit aborted.");
            return Ok(None);
        }
    }

    if config.issue_reference_required.unwrap_or(false) && issue.is_none() {
        println!("{}", "Issue reference is required for commits.".red());
        return Err(anyhow::anyhow!("Aborted: Issue reference required."));
    }

    // Append the issue reference as a trailer/footer if passed
    if config.issue_reference_required.unwrap_or(false) {
        if let Some(issue_ref) = issue {
            commit_message.push_str(&format!("\n\nRefs: {}", issue_ref));
        }
    }

    Ok(Some(commit_message))
}

/// Main function that parses command line arguments and executes the appropriate git operations.
fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let verbose = cli.verbose;
    let config = read_dod_config()?;
    if config.checklist.is_empty() {
        println!("{}", "No checklist items defined.".yellow());
    }

    match cli.command {
        cli::Commands::Status => {
            println!("--- Checking Git status ---");
            let status = git::status(verbose)?;
            println!("{}", format!("Git Status:\n{}", status).blue());
        }
        cli::Commands::Commit { r#type, scope, message, no_verify, issue} => {
            println!("--- Committing changes ---");
            let scope_part = scope.map_or("".to_string(), |s| format!("({})", s));
            let header = format!("{}{}: {}", r#type, scope_part, message);

            let final_commit_message = if no_verify {
                let mut msg = header;
                if let Some(issue_ref) = &issue {
                    msg.push_str(&format!("\n\nRefs: {}", issue_ref));
                }
                Some(msg)
            } else {
                handle_interactive_commit(&config, &header, &issue)?
            };

            if let Some(commit_message) = final_commit_message {
                println!("{}", format!("Commit message will be:\n---\n{}\n---", commit_message).blue());
                git::add_all(verbose)?;
                git::pull_latest_with_rebase(verbose)?;
                git::commit(&commit_message, verbose)?;
                git::push(verbose)?;
                println!("{}", "Successfully committed and pushed changes.".green());
            }
        }
    }
    Ok(())
}