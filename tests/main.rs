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
fn test_project_a() {
    cmd().arg("--input").arg("tests/repositories/project_a").assert().success().stdout(
        predicate::str::contains("github")
            .and(predicate::str::contains("circleci"))
            .and(predicate::str::contains("swift"))
    );
}

#[test]
fn test_project_b() {
    cmd().arg("--input").arg("tests/repositories/project_b").assert().success().stdout(
        predicate::str::contains("cargo")
            .and(predicate::str::contains("rust"))
            .and(predicate::str::contains("toml"))
    );
}

#[test]
fn test_project_a_with_list() {
    cmd()
        .arg("--input")
        .arg("tests/repositories/project_b")
        .arg("--mode")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""cargo""#).and(
            predicate::str::contains(r#""rust""#)).and(
            predicate::str::contains(r#"["#)).and(
            predicate::str::contains(r#"]"#)).and(
            predicate::str::contains(r#""toml""#)
        )
    );
}