use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("loki")
        .expect("binary exists")
        .assert()
        .success();
    Ok(())
}
