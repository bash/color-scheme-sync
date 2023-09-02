#!/usr/bin/env bash

set -e

mkdir -p ~/.local/bin/
cp target/release/color-scheme-sync ~/.local/bin/color-scheme-sync
mkdir -p ~/.config/systemd/user/
envsubst < color-scheme-sync.service.in > ~/.config/systemd/user/color-scheme-sync.service

systemctl enable --now --user color-scheme-sync
