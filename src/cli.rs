use clap::Parser;

/// A CLI to streamline your Git workflow for Trunk-Based Development
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(long)]
    pub verbose: bool,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
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
        /// Optional flag to skip verification of the checklist.
        #[arg(long, default_value_t = false)]
        no_verify: bool,
        /// Optional flag for an issue reference.
        #[arg(long)]
        issue: Option<String>,
    },
}