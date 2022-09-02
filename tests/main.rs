extern crate core;

use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("glowing-happiness").unwrap()
}

#[test]
fn test_show_help() {
    cmd().arg("--help").assert().success().stdout(
        predicate::str::contains("--help")
            .and(predicate::str::contains("--input"))
            .and(predicate::str::contains("--output")),
    );
}

#[test]
fn test_self() {
    cmd().arg("--input").arg(".").assert().success().stdout(
        predicate::str::contains("rust")
    );
}


#[test]
fn test_code_edit() {
    cmd().arg("--input").arg("tests/repositories/CodeEdit").assert().success().stdout(
        predicate::str::contains("github")
            .and(predicate::str::contains("circleci"))
            .and(predicate::str::contains("swift"))
    );
}

#[test]
fn test_helix() {
    cmd().arg("--input").arg("tests/repositories/helix").assert().success().stdout(
        predicate::str::contains("cargo")
            .and(predicate::str::contains("rust"))
            .and(predicate::str::contains("toml"))
    );
}