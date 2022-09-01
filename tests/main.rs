extern crate core;

use std::fs;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;

fn before(input: &PathBuf) {
    fs::copy(PathBuf::from("tests/test.csv"), input).unwrap();
}

fn after(input: &PathBuf, output: &PathBuf) {
    fs::remove_file(input).unwrap();
    fs::remove_file(output).unwrap()
}

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
