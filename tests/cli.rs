use assert_cmd::Command;
use predicates::str::contains;
use std::process;

#[cfg(unix)]
fn command_without_terminal() -> Command {
    use std::os::unix::process::CommandExt;

    let mut command = process::Command::new(assert_cmd::cargo::cargo_bin!("clck"));
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        });
    }
    command.into()
}

#[cfg(windows)]
fn command_without_terminal() -> Command {
    use std::os::windows::process::CommandExt;

    const DETACHED_PROCESS: u32 = 0x0000_0008;
    let mut command = process::Command::new(assert_cmd::cargo::cargo_bin!("clck"));
    command.creation_flags(DETACHED_PROCESS);
    command.into()
}

#[test]
fn help_lists_duration_examples() {
    Command::cargo_bin("clck")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("1H30"))
        .stdout(contains("01:30:00"));
}

#[test]
fn help_lists_scheduling_commands() {
    Command::cargo_bin("clck")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("at"))
        .stdout(contains("from-text"));
}

#[test]
fn piped_text_without_terminal_never_starts_alarm() {
    command_without_terminal()
        .arg("from-text")
        .write_stdin("Meet tomorrow at 9am")
        .assert()
        .failure()
        .stderr(contains("confirmation"))
        .stderr(contains("tomorrow at 9am"));
}

#[test]
fn vague_piped_text_reports_accepted_examples() {
    Command::cargo_bin("clck")
        .unwrap()
        .arg("from-text")
        .write_stdin("Let's talk later after lunch")
        .assert()
        .failure()
        .stderr(contains("2:50pm"))
        .stderr(contains("tomorrow at 9am"));
}
