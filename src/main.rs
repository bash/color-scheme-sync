#![allow(clippy::extra_unused_type_parameters)]

use async_std::stream::{Stream, StreamExt};
use gio::prelude::SettingsExtManual;
use gio::Settings;
use std::error::Error;
use zbus::zvariant::OwnedValue;
use zbus::{dbus_proxy, Connection};

const APPEARANCE_NAMESPACE: &str = "org.freedesktop.appearance";
const COLOR_SCHEME_KEY: &str = "color-scheme";

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;
    let proxy = PortalSettingsProxy::new(&connection).await?;

    set_legacy_gtk_theme_setting(read_color_scheme(&proxy).await?)?;

    eprintln!("Waiting for color scheme changes...");
    let mut color_scheme_updated = color_scheme_changed(&proxy).await?;
    while let Some(color_scheme) = color_scheme_updated.next().await {
        set_legacy_gtk_theme_setting(color_scheme?)?;
    }

    Ok(())
}

async fn read_color_scheme(proxy: &PortalSettingsProxy<'_>) -> Result<ColorScheme, Box<dyn Error>> {
    let value = proxy.Read(APPEARANCE_NAMESPACE, COLOR_SCHEME_KEY).await?;
    Ok(ColorScheme::from(value))
}

async fn color_scheme_changed<'a>(
    proxy: &PortalSettingsProxy<'a>,
) -> Result<impl Stream<Item = Result<ColorScheme, Box<dyn Error + 'a>>>, Box<dyn Error + 'a>> {
    Ok(proxy
        .receive_SettingChanged_with_args(&[(0, APPEARANCE_NAMESPACE), (1, COLOR_SCHEME_KEY)])
        .await?
        .map(|signal| Ok(ColorScheme::from(signal.args()?.value))))
}

fn set_legacy_gtk_theme_setting(color_scheme: ColorScheme) -> Result<(), Box<dyn Error>> {
    let settings = Settings::new("org.gnome.desktop.interface");
    settings.set("gtk-theme", color_scheme.to_legacy_adwaita_theme())?;
    Settings::sync();
    Ok(())
}

#[dbus_proxy(
    interface = "org.freedesktop.portal.Settings",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait PortalSettings {
    fn Read(&self, namespace: &str, key: &str) -> zbus::Result<OwnedValue>;

    #[dbus_proxy(signal)]
    fn SettingChanged(&self, namespace: &str, key: &str, value: OwnedValue) -> Result<()>;
}

#[derive(Debug)]
enum ColorScheme {
    NoPreference,
    PreferDark,
    PreferLight,
}

impl ColorScheme {
    fn to_legacy_adwaita_theme(&self) -> &'static str {
        match self {
            ColorScheme::NoPreference | Self::PreferLight => "Adwaita",
            ColorScheme::PreferDark => "Adwaita-dark",
        }
    }
}

impl From<OwnedValue> for ColorScheme {
    fn from(value: OwnedValue) -> Self {
        // See: https://github.com/flatpak/xdg-desktop-portal/blob/d7a304a00697d7d608821253cd013f3b97ac0fb6/data/org.freedesktop.impl.portal.Settings.xml#L33-L45
        match value.downcast_ref::<u32>() {
            Some(1) => ColorScheme::PreferDark,
            Some(2) => ColorScheme::PreferLight,
            Some(0) | Some(_) | None => ColorScheme::NoPreference,
        }
    }
}
