#![allow(clippy::extra_unused_type_parameters)]

use ashpd::desktop::settings::{ColorScheme, Settings as DesktopSettings};
use async_std::stream::{self, StreamExt as _};
use dedup::StreamExt as _;
use is_executable::IsExecutable as _;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, io};

mod dedup;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = DesktopSettings::new().await?;

    let color_scheme_changed = settings.receive_color_scheme_changed().await?;
    stream::once(settings.color_scheme().await?)
        .chain(color_scheme_changed)
        .dedup()
        .try_for_each(run_commands)
        .await?;

    Ok(())
}

fn run_commands(color_scheme: ColorScheme) -> Result<(), Box<dyn Error>> {
    eprintln!("COLOR_SCHEME={}", to_env_var(color_scheme));

    for command in get_commands()? {
        eprintln!("+ {}", command.display());
        let status = Command::new(&command)
            .env("COLOR_SCHEME", to_env_var(color_scheme))
            .status();
        match status {
            Err(error) => eprintln!("command {} failed: {error:?}", command.display()),
            Ok(status) if !status.success() => eprintln!(
                "command {} failed with exit code {}",
                command.display(),
                status
            ),
            Ok(_) => {}
        }
    }
    Ok(())
}

fn to_env_var(color_scheme: ColorScheme) -> &'static str {
    match color_scheme {
        ColorScheme::NoPreference => "no-preference",
        ColorScheme::PreferDark => "prefer-dark",
        ColorScheme::PreferLight => "prefer-light",
    }
}

fn get_commands() -> io::Result<Vec<PathBuf>> {
    match commands_dir() {
        None => Ok(Vec::new()),
        Some(dir) => Ok(fs::read_dir(dir)?
            .filter_map(|r| r.ok())
            .map(|e| e.path())
            .filter(|p| p.is_executable())
            .collect()),
    }
}

fn commands_dir() -> Option<PathBuf> {
    let mut dir = dirs::config_dir()?;
    dir.push("color-scheme-sync.d");
    dir.exists().then_some(dir)
}
