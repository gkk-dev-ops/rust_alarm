use anyhow::Result;

pub fn notification_body(target: Option<&str>, title: Option<&str>) -> String {
    match (title, target) {
        (Some(title), _) => format!("Time is up for {title}."),
        (None, Some(target)) => format!("Time is up for {target}."),
        (None, None) => "Time is up!".to_owned(),
    }
}

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
