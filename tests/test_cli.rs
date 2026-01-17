use assert_cmd::{assert::OutputAssertExt, cargo};
use std::process::Command;

fn command() -> Command {
    Command::new(cargo::cargo_bin!("rust-cli-template"))
}

#[test]
fn test_greet() {
    command()
        .arg("Jon")
        .assert()
        .success()
        .stdout("Hello, Jon!\n");
}
