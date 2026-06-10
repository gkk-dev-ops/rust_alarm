use crate::{
    audio::{AlarmPlayer, ResolvedSound},
    display::{DisplayEvent, TerminalSession},
    notification,
    timer::Countdown,
};
use anyhow::Result;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

#[derive(Clone, Debug)]
pub struct AlarmRequest {
    pub duration: Duration,
    pub font: String,
    pub sound_name: String,
    pub sound: ResolvedSound,
    pub notification: bool,
}

pub fn run_alarm(request: AlarmRequest) -> Result<()> {
    let cancelled = Arc::new(AtomicBool::new(false));
    let signal = Arc::clone(&cancelled);
    ctrlc::set_handler(move || signal.store(true, Ordering::SeqCst))?;

    let terminal = TerminalSession::enter()?;
    let timer = Countdown::new(Instant::now(), request.duration);
    let mut displayed_second = None;
    loop {
        let remaining = timer.remaining(Instant::now());
        let current_second = remaining.as_secs() + u64::from(remaining.subsec_nanos() > 0);
        if displayed_second != Some(current_second) {
            terminal.render_countdown(remaining, &request.font, &request.sound_name)?;
            displayed_second = Some(current_second);
        }
        if timer.is_finished(Instant::now()) {
            break;
        }
        if cancelled.load(Ordering::SeqCst) {
            return Ok(());
        }
        match TerminalSession::next_event(Duration::from_millis(100), false)? {
            DisplayEvent::Cancel => return Ok(()),
            DisplayEvent::Resize => displayed_second = None,
            _ => {}
        }
    }

    if request.notification {
        let _ = notification::notify_time_up();
    }
    terminal.render_ringing()?;
    let player = AlarmPlayer::start(&request.sound)?;
    let mut last_bell = Instant::now();
    loop {
        if cancelled.load(Ordering::SeqCst)
            || TerminalSession::next_event(Duration::from_millis(100), true)?
                == DisplayEvent::Dismiss
        {
            break;
        }
        if last_bell.elapsed() >= Duration::from_secs(1) {
            player.bell();
            last_bell = Instant::now();
        }
    }
    Ok(())
}
