use std::process::{Command, Output};
use std::error::Error;
use std::str;

pub fn commit_and_push() -> Result<(), Box<dyn Error>> {
    // generate a key after repo is made 'ssh-keygen -t ed25519 -C "jackmaun@colostate.edu"'
    Command::new("git")
        .args(&["remote", "set-url", "origin", "git@github.com:jackmaun/security-fixer"])
        .status()?;

    Command::new("git")
        .args(&["checkout", "-b", "auto-fix"])
        .status()?;

    Command::new("git")
        .args(&["add", "."])
        .status()?;

    Command::new("git")
        .args(&["commit", "-m", "AI Auto-Fixed Security Vulnerabilities"])
        .status()?;

    Command::new("git")
        .args(&["fetch", "origin", "main"])
        .status()?;

    let merge_output: Output = Command::new("git")
        .args(&["merge", "--no-commit", "--no-ff", "origin/main"])
        .output()?;

    let merge_stdout = str::from_utf8(&merge_output.stdout)?;
    let merge_stderr = str::from_utf8(&merge_output.stderr)?;

    if merge_stderr.contains("CONFLICT") {
        println!("[-] Merge conflict found... Aborting merge.");
        Command::new("git")
            .args(&["merge", "--abort"])
            .status()?;
        return Err("[-] Merge conflict detected. Resolve conflicts.".into());
    }

    Command::new("git")
        .args(&["push", "origin", "auto-fix"])
        .status()?;

    Command::new("gh")
        .args(&["pr", "create", "--title", "AI Security Fix", "--body", "This PR contains AI-generated security fixes."])
        .status()?;

    Command::new("gh")
        .args(&["pr", "merge", "--auto", "--squash"])
        .status()?;

    Ok(())
}
