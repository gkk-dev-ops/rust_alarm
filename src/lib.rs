pub mod app;
pub mod audio;
pub mod cli;
pub mod config;
pub mod display;
pub mod fonts;
pub mod notification;
pub mod timer;

pub const APP_NAME: &str = "alarm-clock";

pub fn run() -> anyhow::Result<()> {
    use anyhow::{bail, Context};
    use clap::Parser;
    use config::{Config, Overrides, SoundSetting};
    use inquire::Confirm;

    let cli = cli::Cli::parse();
    let config = Config::load()?;

    if let Some(command) = cli.command {
        match command {
            cli::Command::Fonts => {
                let catalog = fonts::FontCatalog::default();
                for name in catalog.names() {
                    println!("{name}");
                    if let Some(font) = catalog.by_name(name) {
                        for line in font.render("01:30") {
                            println!("{line}");
                        }
                    }
                    println!();
                }
            }
            cli::Command::Sounds => {
                for (name, path) in audio::discover_sounds_in(&audio::system_sound_directories())? {
                    println!("{name}: {}", path.display());
                }
            }
            cli::Command::Config { show, reset } => {
                if reset {
                    if Confirm::new("Reset saved alarm-clock settings?")
                        .with_default(false)
                        .prompt()?
                    {
                        Config::default().save()?;
                        println!("Configuration reset.");
                    }
                } else if show {
                    println!("{}", toml::to_string_pretty(&config)?);
                } else {
                    println!("Configuration: {}", Config::path()?.display());
                    println!("{}", toml::to_string_pretty(&config)?);
                }
            }
        }
        return Ok(());
    }

    let (duration, effective) = if let Some(duration) = cli.duration {
        let duration = cli::parse_duration(&duration)?;
        let effective = config.resolve(Overrides {
            font: cli.font,
            notification: cli.no_notification.then_some(false),
            sound: cli.sound.map(SoundSetting::File),
        });
        (duration, effective)
    } else {
        let answers = cli::prompt_for_alarm(&config)?;
        let effective = Config {
            font: answers.font,
            notification: answers.notification,
            sound: answers.sound,
        };
        if answers.save_defaults {
            effective.save()?;
        }
        (answers.duration, effective)
    };

    let (sound_name, sound) = match &effective.sound {
        SoundSetting::System(name) => (name.clone(), audio::resolve_system_sound(name)?),
        SoundSetting::File(path) => (
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("custom sound")
                .to_owned(),
            audio::resolve_custom_sound(path)?,
        ),
        SoundSetting::TerminalBell => ("terminal bell".into(), audio::ResolvedSound::TerminalBell),
    };
    if duration.is_zero() {
        bail!("duration must be greater than zero");
    }

    app::run_alarm(app::AlarmRequest {
        duration,
        font: effective.font,
        sound_name,
        sound,
        notification: effective.notification,
    })
    .context("alarm failed")
}

#[cfg(test)]
mod tests {
    #[test]
    fn package_name_is_stable() {
        assert_eq!(super::APP_NAME, "alarm-clock");
    }
}
