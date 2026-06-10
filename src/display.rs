use crate::fonts::{FontCatalog, Size};
use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    io::{self, Write},
    time::Duration,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayEvent {
    None,
    Cancel,
    Dismiss,
    Resize,
}

pub fn event_from_key(key: KeyEvent, ringing: bool) -> DisplayEvent {
    if ringing {
        return DisplayEvent::Dismiss;
    }
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => DisplayEvent::Cancel,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => DisplayEvent::Cancel,
        _ => DisplayEvent::None,
    }
}

pub struct TerminalSession;

impl TerminalSession {
    pub fn enter() -> Result<Self> {
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, Hide)?;
        Ok(Self)
    }

    pub fn next_event(timeout: Duration, ringing: bool) -> Result<DisplayEvent> {
        if !event::poll(timeout)? {
            return Ok(DisplayEvent::None);
        }
        Ok(match event::read()? {
            Event::Key(key) => event_from_key(key, ringing),
            Event::Resize(_, _) => DisplayEvent::Resize,
            _ => DisplayEvent::None,
        })
    }

    pub fn render_countdown(
        &self,
        remaining: Duration,
        preferred_font: &str,
        sound: &str,
        target: Option<&str>,
    ) -> Result<()> {
        let (width, height) = terminal::size()?;
        let text = format_duration(remaining);
        let catalog = FontCatalog::default();
        let available = Size::new(width.saturating_sub(2), height.saturating_sub(4));
        let lines = catalog
            .largest_fit_preferring(preferred_font, &text, available)
            .map(|font| font.render(&text))
            .unwrap_or_else(|| vec![text]);
        let full_status = countdown_status(sound, target);
        let compact_status = countdown_status(sound, None);
        let status = if full_status.chars().count() <= usize::from(width) {
            Some(full_status.as_str())
        } else if compact_status.chars().count() <= usize::from(width) {
            Some(compact_status.as_str())
        } else {
            None
        };
        render_lines(&lines, status)
    }

    pub fn render_ringing(&self, target: Option<&str>) -> Result<()> {
        let status = target
            .map(|target| format!("Target: {target} | Press any key to dismiss"))
            .unwrap_or_else(|| "Press any key to dismiss".to_owned());
        render_lines(&["TIME IS UP!".to_owned()], Some(&status))
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

fn render_lines(lines: &[String], status: Option<&str>) -> Result<()> {
    let mut stdout = io::stdout();
    queue!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
    for line in lines {
        queue!(stdout, Print(line), Print("\r\n"))?;
    }
    if let Some(status) = status {
        queue!(stdout, Print("\r\n"), Print(status))?;
    }
    stdout.flush()?;
    Ok(())
}

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs() + u64::from(duration.subsec_nanos() > 0);
    let hours = seconds / 3_600;
    let minutes = (seconds % 3_600) / 60;
    let seconds = seconds % 60;
    if hours > 0 {
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}

pub fn countdown_status(sound: &str, target: Option<&str>) -> String {
    match target {
        Some(target) => {
            format!("Target: {target} | Sound: {sound} | q/Esc/Ctrl+C to cancel")
        }
        None => format!("Sound: {sound} | q/Esc/Ctrl+C to cancel"),
    }
}

#[cfg(test)]
mod tests {
    use super::{countdown_status, event_from_key, DisplayEvent};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn maps_countdown_cancel_keys() {
        assert_eq!(
            event_from_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE), false),
            DisplayEvent::Cancel
        );
        assert_eq!(
            event_from_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE), false),
            DisplayEvent::Cancel
        );
        assert_eq!(
            event_from_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE), true),
            DisplayEvent::Dismiss
        );
    }

    #[test]
    fn countdown_status_includes_optional_target() {
        assert_eq!(
            countdown_status("Glass", Some("2026-06-11 09:00 EDT")),
            "Target: 2026-06-11 09:00 EDT | Sound: Glass | q/Esc/Ctrl+C to cancel"
        );
        assert_eq!(
            countdown_status("Glass", None),
            "Sound: Glass | q/Esc/Ctrl+C to cancel"
        );
    }
}
