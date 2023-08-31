#![allow(clippy::extra_unused_type_parameters)]

use ashpd::desktop::settings::{ColorScheme, Settings as DesktopSettings};
use async_std::stream::{self, StreamExt as _};
use dedup::StreamExt as _;
use gio::prelude::SettingsExtManual as _;
use gio::Settings;
use std::error::Error;

mod dedup;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = DesktopSettings::new().await?;

    eprintln!("Waiting for color scheme changes...");
    let color_scheme_changed = settings.receive_color_scheme_changed().await?;
    stream::once(settings.color_scheme().await?)
        .chain(color_scheme_changed)
        .dedup()
        .try_for_each(set_legacy_gtk_theme_setting)
        .await?;

    Ok(())
}

fn set_legacy_gtk_theme_setting(color_scheme: ColorScheme) -> Result<(), Box<dyn Error>> {
    let settings = Settings::new("org.gnome.desktop.interface");
    settings.set("gtk-theme", to_legacy_adwaita_theme(color_scheme))?;
    Settings::sync();
    Ok(())
}

fn to_legacy_adwaita_theme(color_scheme: ColorScheme) -> &'static str {
    match color_scheme {
        ColorScheme::NoPreference | ColorScheme::PreferLight => "Adwaita",
        ColorScheme::PreferDark => "Adwaita-dark",
    }
}
