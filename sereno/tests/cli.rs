use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn shows_help() {
    cargo_bin_cmd!("sereno")
        .arg("--help")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Usage:")
                .and(predicate::str::contains("version"))
                .and(predicate::str::contains("doctor")),
        );
}

#[test]
fn prints_json_version() {
    cargo_bin_cmd!("sereno")
        .args(["--format", "json", "version"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\"success\":true")
                .and(predicate::str::contains("\"name\":\"sereno\""))
                .and(predicate::str::contains("\"version\":\"0.1.0\"")),
        );
}

#[test]
fn rejects_invalid_format() {
    cargo_bin_cmd!("sereno")
        .args(["--format", "yaml", "version"])
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("invalid value"));
}
