#![allow(clippy::extra_unused_type_parameters)]

use ashpd::desktop::settings::{ColorScheme, Settings as DesktopSettings};
use async_std::stream::StreamExt as _;
use gio::prelude::SettingsExtManual as _;
use gio::Settings;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = DesktopSettings::new().await?;

    set_legacy_gtk_theme_setting(settings.color_scheme().await?)?;

    eprintln!("Waiting for color scheme changes...");
    let mut color_scheme_updated = settings.receive_color_scheme_changed().await?;
    while let Some(color_scheme) = color_scheme_updated.next().await {
        set_legacy_gtk_theme_setting(color_scheme)?;
    }

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
