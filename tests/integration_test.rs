use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("lokisay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("lokisay")
        .expect("binary exists")
        .args(&["-f", "~/file_doesnt_exist.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}
