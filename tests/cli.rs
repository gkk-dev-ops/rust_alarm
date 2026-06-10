use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn help_lists_duration_examples() {
    Command::cargo_bin("alarm-clock")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("1H30"))
        .stdout(contains("01:30:00"));
}
