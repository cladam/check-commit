# Archived and implemented in tbdflow


# check-commit

A TBD-friendly pre-commit tool that enforces your team's Definition of Done (DoD) using an interactive checklist.

This tool is designed for teams practicing Trunk-Based Development who want to automate their quality standards without the friction of a formal Pull Request process. It makes your team's DoD an active, developer-centric part of the commit workflow.

The long-term goal is to integrate this functionality directly into the [`tbdflow`](https://github.com/cladam/tbdflow) CLI.

# How It Works
`check-commit` is a smart wrapper around `git commit`. Before it executes the commit, it looks for a `.dod.yml` file in your repository's root.

If found, it presents an interactive checklist to the developer in the terminal. If the developer confirms the checklist, the tool proceeds to execute the `git commit` command with the original arguments.

This creates a lightweight, non-obtrusive quality gate right at the moment of commit.

## Configuration (`.dod.yml`)
The checklist is configured in a `.dod.yml` file in your project's root. This makes your team's Definition of Done a version-controlled artifact that lives with your code.

**Example** `.dod.yml`:
```yaml
# --- Optional Issue Tracker Integration ---
# If true, the tool will require the --issue <ID> flag to be used,
# ensuring all work is traceable.
issue_reference_required: true

# --- Interactive Checklist ---
# This list is presented to the developer before every commit.
checklist:
  - "Code is clean, readable, and adheres to team coding standards."
  - "All relevant automated tests (unit, integration) pass successfully."
  - "New features or bug fixes are covered by appropriate new tests."
  - "Security implications of this change have been considered."
  - "Relevant documentation (code comments, READMEs, etc.) is updated."
```

If you try to proceed without checking all items, the tool will offer to add a TODO list to your commit message footer, 
ensuring the incomplete work is tracked directly in your Git history.

All of this can be bypassed by the flag `--no-verify`.
It answers the question, "For this one specific commit, do I have a good reason to bypass our default rules?"

## Getting Started (Development)
This project is currently in the planning and early development phase.

The initial proof-of-concept will be built in Rust.

Roadmap:

* ~~Parse command-line arguments for `git commit`.~~
* ~~Read and parse the `.dod.yml` configuration file.~~
* ~~Implement the interactive checklist using a TUI library.~~
* ~~Execute `git commit` upon successful confirmation.~~
* ~~Integrate this logic into the `tbdflow commit` command.~~
