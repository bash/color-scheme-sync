#!/bin/sh

if [ "$COLOR_SCHEME" = "prefer-light" ]; then
    gtk_theme=Adwaita
else
    gtk_theme=Adwaita-dark
fi

gsettings set org.gnome.desktop.interface gtk-theme "$gtk_theme"
