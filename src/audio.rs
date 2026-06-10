use anyhow::{bail, Context, Result};
#[cfg(feature = "native-audio")]
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
#[cfg(feature = "native-audio")]
use std::process::Command;
#[cfg(feature = "native-audio")]
use tempfile::NamedTempFile;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackBackend {
    Native,
    Ffmpeg,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResolvedSound {
    Native(PathBuf),
    Ffmpeg(PathBuf),
    TerminalBell,
}

pub fn classify_custom_sound(path: &Path) -> Result<PlaybackBackend> {
    if !path.is_file() {
        bail!(
            "sound file does not exist or is unreadable: {}",
            path.display()
        );
    }
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    Ok(match extension.as_str() {
        "mp3" | "wav" | "flac" | "ogg" | "oga" | "aiff" | "aif" => PlaybackBackend::Native,
        _ => PlaybackBackend::Ffmpeg,
    })
}

pub fn discover_sounds_in(directories: &[PathBuf]) -> Result<BTreeMap<String, PathBuf>> {
    let mut sounds = BTreeMap::new();
    for directory in directories {
        if !directory.exists() {
            continue;
        }
        visit_sounds(directory, &mut sounds)?;
    }
    Ok(sounds)
}

fn visit_sounds(directory: &Path, sounds: &mut BTreeMap<String, PathBuf>) -> Result<()> {
    for entry in fs::read_dir(directory)
        .with_context(|| format!("could not inspect sound directory {}", directory.display()))?
    {
        let path = entry?.path();
        if path.is_dir() {
            visit_sounds(&path, sounds)?;
        } else if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
            sounds.entry(stem.to_owned()).or_insert(path);
        }
    }
    Ok(())
}

pub fn system_sound_directories() -> Vec<PathBuf> {
    let mut directories = Vec::new();
    #[cfg(target_os = "macos")]
    {
        directories.push(PathBuf::from("/System/Library/Sounds"));
        if let Some(home) = std::env::var_os("HOME") {
            directories.push(PathBuf::from(home).join("Library/Sounds"));
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
            directories.push(PathBuf::from(data_home).join("sounds"));
        }
        if let Some(home) = std::env::var_os("HOME") {
            directories.push(PathBuf::from(home).join(".local/share/sounds"));
        }
        directories.push(PathBuf::from("/usr/share/sounds"));
    }
    directories
}

pub fn resolve_system_sound(name: &str) -> Result<ResolvedSound> {
    let sounds = discover_sounds_in(&system_sound_directories())?;
    let found = sounds
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
        .or_else(|| {
            sounds.iter().find(|(candidate, _)| {
                let name = candidate.to_ascii_lowercase();
                name.contains("alarm") || name.contains("complete") || name.contains("notification")
            })
        });
    Ok(found
        .map(|(_, path)| ResolvedSound::Native(path.clone()))
        .unwrap_or(ResolvedSound::TerminalBell))
}

pub fn resolve_custom_sound(path: &Path) -> Result<ResolvedSound> {
    Ok(match classify_custom_sound(path)? {
        PlaybackBackend::Native => ResolvedSound::Native(path.to_owned()),
        PlaybackBackend::Ffmpeg => ResolvedSound::Ffmpeg(path.to_owned()),
    })
}

#[cfg(feature = "native-audio")]
pub struct AlarmPlayer {
    _stream: Option<OutputStream>,
    sink: Option<Sink>,
    _converted: Option<NamedTempFile>,
}

#[cfg(feature = "native-audio")]
impl AlarmPlayer {
    pub fn start(sound: &ResolvedSound) -> Result<Self> {
        match sound {
            ResolvedSound::TerminalBell => {
                print!("\x07");
                Ok(Self {
                    _stream: None,
                    sink: None,
                    _converted: None,
                })
            }
            ResolvedSound::Native(path) => Self::start_native(path, None),
            ResolvedSound::Ffmpeg(path) => {
                let converted = NamedTempFile::with_suffix(".wav")?;
                let status = Command::new("ffmpeg")
                    .args(["-y", "-nostdin", "-loglevel", "error", "-i"])
                    .arg(path)
                    .arg(converted.path())
                    .status()
                    .context("FFmpeg is required to play this audio format")?;
                if !status.success() {
                    bail!("FFmpeg could not decode {}", path.display());
                }
                let converted_path = converted.path().to_owned();
                Self::start_native(&converted_path, Some(converted))
            }
        }
    }

    fn start_native(path: &Path, converted: Option<NamedTempFile>) -> Result<Self> {
        let mut stream =
            OutputStreamBuilder::open_default_stream().context("could not open audio output")?;
        stream.log_on_drop(false);
        let sink = Sink::connect_new(stream.mixer());
        let file = fs::File::open(path)
            .with_context(|| format!("could not open sound {}", path.display()))?;
        let decoder = Decoder::try_from(file)
            .with_context(|| format!("could not decode sound {}", path.display()))?;
        sink.append(decoder.repeat_infinite());
        Ok(Self {
            _stream: Some(stream),
            sink: Some(sink),
            _converted: converted,
        })
    }

    pub fn bell(&self) {
        if self.sink.is_none() {
            print!("\x07");
        }
    }
}

#[cfg(feature = "native-audio")]
impl Drop for AlarmPlayer {
    fn drop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }
}

#[cfg(not(feature = "native-audio"))]
pub struct AlarmPlayer;

#[cfg(not(feature = "native-audio"))]
impl AlarmPlayer {
    pub fn start(_sound: &ResolvedSound) -> Result<Self> {
        print!("\x07");
        Ok(Self)
    }

    pub fn bell(&self) {
        print!("\x07");
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_custom_sound, discover_sounds_in, PlaybackBackend};
    use std::fs;

    #[test]
    fn discovers_logical_sound_names() {
        let directory = tempfile::tempdir().unwrap();
        fs::write(directory.path().join("Glass.aiff"), []).unwrap();
        let sounds = discover_sounds_in(&[directory.path().to_path_buf()]).unwrap();
        assert_eq!(sounds["Glass"].file_name().unwrap(), "Glass.aiff");
    }

    #[test]
    fn classifies_native_and_ffmpeg_audio() {
        let directory = tempfile::tempdir().unwrap();
        let mp3 = directory.path().join("alarm.MP3");
        let mp4 = directory.path().join("alarm.mp4");
        fs::write(&mp3, []).unwrap();
        fs::write(&mp4, []).unwrap();
        assert_eq!(
            classify_custom_sound(&mp3).unwrap(),
            PlaybackBackend::Native
        );
        assert_eq!(
            classify_custom_sound(&mp4).unwrap(),
            PlaybackBackend::Ffmpeg
        );
    }
}
