# color-scheme-sync

☀️ 🌑

A daemon that watches for changes in the new system-wide
[dark style preference](https://blogs.gnome.org/alexm/2021/10/04/dark-style-preference/) in GNOME \
mirroring your preference to the legacy `gtk-theme` preference.


## Why?
Electron [does not support](https://github.com/electron/electron/issues/33635) the new dark style preference, which means that apps such as [VSCode](https://github.com/microsoft/vscode/issues/146804) do not support it either.

This annoys me as I like switching between dark and light mode.

## Installing
* Clone this repository
* Run `cargo build --release` 
* Run `./install.sh`. \
  This will install a systemd service that waits for changes to your color scheme preference,
  automatically any changes to the legacy `gtk-theme` preference.