use anyhow::Result;

pub fn notify_time_up() -> Result<()> {
    notify_rust::Notification::new()
        .summary("Alarm clock")
        .body("Time is up!")
        .show()?;
    Ok(())
}
