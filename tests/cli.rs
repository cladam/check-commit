use assert_cmd::Command;
use predicates::str::contains;

mod utils;
use utils::setup_temp_git_repo;

/// Tests that the status command outputs the expected status message.
#[test]
fn test_status_command() {
    let mut cmd = Command::cargo_bin("check-commit").unwrap();
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(contains("Git Status"));
}

/// Tests that adding a new file and committing it with the commit command works correctly.
#[test]
fn test_commit_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();

    // Create a file to commit
    std::fs::write(repo_path.join(".dod.yml"), r#"
issue_reference_required: false
checklist:
  - "Code is clean, readable, and adheres to team coding standards."
  - "All relevant automated tests (unit, integration) pass successfully."
  - "New features or bug fixes are covered by appropriate new tests."
  - "Security implications of this change have been considered."
  - "Relevant documentation (code comments, READMEs, etc.) is updated."
"#).unwrap();

    // Stage the file
    Command::new("git")
        .arg("add")
        .arg(".dod.yml")
        .current_dir(&repo_path)
        .output()
        .unwrap();

    // Wait until the working directory is clean
    let mut retries = 5;
    while retries > 0 {
        let status = Command::new("git")
            .args(&["status", "--porcelain"])
            .current_dir(&repo_path)
            .output()
            .unwrap();
        if status.stdout.is_empty() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        retries -= 1;
    }

    let mut cmd = Command::cargo_bin("check-commit").unwrap();
    // Run the commit command with a feature type, scope and message
    cmd.arg("commit")
        .arg("--type").arg("feat")
        .arg("--scope").arg("ui")
        .arg("--message").arg("Add new button")
        .arg("--no-verify");
    cmd.assert()
        .success()
        .stdout(contains("Successfully committed and pushed changes."));

}