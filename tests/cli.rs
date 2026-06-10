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

#[test]
fn help_lists_scheduling_commands() {
    Command::cargo_bin("alarm-clock")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("at"))
        .stdout(contains("from-text"));
}

#[test]
fn piped_text_without_terminal_never_starts_alarm() {
    Command::cargo_bin("alarm-clock")
        .unwrap()
        .arg("from-text")
        .write_stdin("Meet tomorrow at 9am")
        .assert()
        .failure()
        .stderr(contains("confirmation"))
        .stderr(contains("tomorrow at 9am"));
}

#[test]
fn vague_piped_text_reports_accepted_examples() {
    Command::cargo_bin("alarm-clock")
        .unwrap()
        .arg("from-text")
        .write_stdin("Let's talk later after lunch")
        .assert()
        .failure()
        .stderr(contains("2:50pm"))
        .stderr(contains("tomorrow at 9am"));
}
