#!/usr/bin/env bash

sed -i "s/@@HOME_DIR@@/$HOME/g" color-scheme-sync.service

cp target/release/color-scheme-sync ~/.local/bin/color-scheme-sync
cp color-scheme-sync.service ~/.config/systemd/user/

systemctl enable --now --user color-scheme-sync