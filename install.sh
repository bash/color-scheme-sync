#!/usr/bin/env bash

set -e

sed -i "s#@@HOME_DIR@@#$HOME#g" color-scheme-sync.service

mkdir -p ~/.local/bin/
cp target/release/color-scheme-sync ~/.local/bin/color-scheme-sync
mkdir -p ~/.config/systemd/user/
cp color-scheme-sync.service ~/.config/systemd/user/

systemctl enable --now --user color-scheme-sync
