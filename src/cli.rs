use crate::{
    audio::{discover_sounds_in, system_sound_directories},
    config::{Config, SoundSetting},
    fonts::FontCatalog,
};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use inquire::{Confirm, Select, Text};
use std::{path::PathBuf, time::Duration};
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(
    name = "alarm-clock",
    about = "Responsive cross-platform countdown alarm"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
    #[arg(value_name = "DURATION", help = "Examples: 45s, 1h30m, 1H30, 01:30:00")]
    pub duration: Option<String>,
    #[arg(long)]
    pub sound: Option<PathBuf>,
    #[arg(long)]
    pub font: Option<String>,
    #[arg(long)]
    pub no_notification: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Config {
        #[arg(long, conflicts_with = "reset")]
        show: bool,
        #[arg(long, conflicts_with = "show")]
        reset: bool,
    },
    Fonts,
    Sounds,
}

#[derive(Clone, Debug)]
pub struct InteractiveAnswers {
    pub duration: String,
    pub font: String,
    pub notification: bool,
    pub sound: SoundSetting,
    pub save_defaults: bool,
}

#[derive(Clone, Debug)]
pub struct ValidatedInteractiveAnswers {
    pub duration: Duration,
    pub font: String,
    pub notification: bool,
    pub sound: SoundSetting,
    pub save_defaults: bool,
}

impl InteractiveAnswers {
    pub fn validate(self) -> Result<ValidatedInteractiveAnswers> {
        Ok(ValidatedInteractiveAnswers {
            duration: parse_duration(&self.duration)?,
            font: self.font,
            notification: self.notification,
            sound: self.sound,
            save_defaults: self.save_defaults,
        })
    }
}

pub fn prompt_for_alarm(config: &Config) -> Result<ValidatedInteractiveAnswers> {
    let duration = Text::new("Duration (for example 1H30, 10m, or 45s):")
        .prompt()
        .context("interactive duration prompt was cancelled")?;
    let fonts: Vec<_> = FontCatalog::default().names().map(str::to_owned).collect();
    let default_font = fonts
        .iter()
        .position(|font| font == &config.font)
        .unwrap_or(0);
    let font = Select::new("ASCII font:", fonts)
        .with_starting_cursor(default_font)
        .prompt()?;
    let sounds = discover_sounds_in(&system_sound_directories())?;
    let sound_names: Vec<_> = sounds.keys().cloned().collect();
    let sound = if sound_names.is_empty() {
        SoundSetting::TerminalBell
    } else {
        SoundSetting::System(Select::new("Alarm sound:", sound_names).prompt()?)
    };
    let notification = Confirm::new("Show desktop notification?")
        .with_default(config.notification)
        .prompt()?;
    let save_defaults = Confirm::new("Save these settings as defaults?")
        .with_default(true)
        .prompt()?;
    InteractiveAnswers {
        duration,
        font,
        notification,
        sound,
        save_defaults,
    }
    .validate()
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("invalid duration `{input}`; use formats such as 45s, 1h30m, 1H30, or 01:30:00")]
pub struct DurationParseError {
    input: String,
}

pub fn parse_duration(input: &str) -> Result<Duration, DurationParseError> {
    let input = input.trim();
    parse_colon(input)
        .or_else(|| parse_units(input))
        .or_else(|| parse_compact_hours(input))
        .filter(|duration| !duration.is_zero())
        .ok_or_else(|| DurationParseError {
            input: input.to_owned(),
        })
}

fn parse_colon(input: &str) -> Option<Duration> {
    let parts: Vec<_> = input.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let hours = parts[0].parse::<u64>().ok()?;
    let minutes = parts[1].parse::<u64>().ok()?;
    let seconds = parts[2].parse::<u64>().ok()?;
    (minutes < 60 && seconds < 60)
        .then(|| Duration::from_secs(hours * 3_600 + minutes * 60 + seconds))
}

fn parse_units(input: &str) -> Option<Duration> {
    let lower = input.to_ascii_lowercase();
    let mut number = String::new();
    let mut seconds = 0_u64;
    let mut seen = [false; 3];

    for character in lower.chars() {
        if character.is_ascii_digit() {
            number.push(character);
            continue;
        }
        let index = match character {
            'h' => 0,
            'm' => 1,
            's' => 2,
            _ => return None,
        };
        if seen[index] || number.is_empty() {
            return None;
        }
        seen[index] = true;
        let value = number.parse::<u64>().ok()?;
        seconds = seconds.checked_add(value.checked_mul([3_600, 60, 1][index])?)?;
        number.clear();
    }

    (number.is_empty() && seen.iter().any(|seen| *seen)).then(|| Duration::from_secs(seconds))
}

fn parse_compact_hours(input: &str) -> Option<Duration> {
    let lower = input.to_ascii_lowercase();
    let (hours, minutes) = lower.split_once('h')?;
    if hours.is_empty() || minutes.is_empty() || minutes.len() > 2 {
        return None;
    }
    let hours = hours.parse::<u64>().ok()?;
    let minutes = minutes.parse::<u64>().ok()?;
    (minutes < 60).then(|| Duration::from_secs(hours * 3_600 + minutes * 60))
}

#[cfg(test)]
mod tests {
    use super::{parse_duration, InteractiveAnswers};
    use crate::config::SoundSetting;
    use std::time::Duration;

    #[test]
    fn parses_supported_duration_formats() {
        assert_eq!(parse_duration("45s").unwrap(), Duration::from_secs(45));
        assert_eq!(parse_duration("10m").unwrap(), Duration::from_secs(600));
        assert_eq!(parse_duration("1h30m").unwrap(), Duration::from_secs(5_400));
        assert_eq!(parse_duration("1H30").unwrap(), Duration::from_secs(5_400));
        assert_eq!(
            parse_duration("01:30:00").unwrap(),
            Duration::from_secs(5_400)
        );
    }

    #[test]
    fn rejects_invalid_duration_formats() {
        assert!(parse_duration("1H75").is_err());
        assert!(parse_duration("nonsense").is_err());
        assert!(parse_duration("00:75:00").is_err());
    }

    #[test]
    fn validates_interactive_duration() {
        let answers = InteractiveAnswers {
            duration: "1H30".into(),
            font: "standard".into(),
            notification: true,
            sound: SoundSetting::System("Glass".into()),
            save_defaults: false,
        };
        assert_eq!(
            answers.validate().unwrap().duration,
            Duration::from_secs(5_400)
        );
    }
}
