mod cli;
mod git;
use clap::Parser;
use colored::Colorize;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Status => {
            println!("--- Checking Git status ---");
            let status = git::status()?;
            println!("{}", format!("Git Status:\n{}", status).green());
        }
        cli::Commands::Commit { r#type, scope, message} => {
            println!("--- Committing changes ---");
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