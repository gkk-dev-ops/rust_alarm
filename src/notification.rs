use anyhow::Result;

pub fn notification_body(target: Option<&str>) -> String {
    target
        .map(|target| format!("Time is up for {target}."))
        .unwrap_or_else(|| "Time is up!".to_owned())
}

pub fn notify_time_up(target: Option<&str>) -> Result<()> {
    notify_rust::Notification::new()
        .summary("Alarm clock")
        .body(&notification_body(target))
        .show()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::notification_body;

    #[test]
    fn notification_body_includes_optional_target() {
        assert_eq!(
            notification_body(Some("2026-06-11 09:00 EDT")),
            "Time is up for 2026-06-11 09:00 EDT."
        );
        assert_eq!(notification_body(None), "Time is up!");
    }
}
