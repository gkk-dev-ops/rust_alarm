use anyhow::Result;

pub fn notification_body(target: Option<&str>, title: Option<&str>) -> String {
    match (title, target) {
        (Some(title), _) => format!("Time is up for {title}."),
        (None, Some(target)) => format!("Time is up for {target}."),
        (None, None) => "Time is up!".to_owned(),
    }
}

#[cfg(target_os = "macos")]
pub fn notify_time_up(target: Option<&str>, title: Option<&str>) -> Result<()> {
    let body = notification_body(target, title);
    // Escape for AppleScript double-quoted string
    let escaped = body.replace('\\', "\\\\").replace('"', "\\\"");
    let script = format!("display notification \"{escaped}\" with title \"Alarm clock\"");
    // osascript goes through the Automation subsystem, which is reliably
    // allowed to post notifications from CLI tools without a bundle identity.
    let _ = std::process::Command::new("osascript")
        .args(["-e", &script])
        .status();
    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub fn notify_time_up(target: Option<&str>, title: Option<&str>) -> Result<()> {
    notify_rust::Notification::new()
        .summary("Alarm clock")
        .body(&notification_body(target, title))
        .show()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::notification_body;

    #[test]
    fn notification_body_includes_optional_target_and_title() {
        assert_eq!(
            notification_body(Some("2026-06-11 09:00 EDT"), Some("Lunch")),
            "Time is up for Lunch."
        );
        assert_eq!(
            notification_body(Some("2026-06-11 09:00 EDT"), None),
            "Time is up for 2026-06-11 09:00 EDT."
        );
        assert_eq!(
            notification_body(None, Some("Lunch")),
            "Time is up for Lunch."
        );
        assert_eq!(notification_body(None, None), "Time is up!");
    }
}
