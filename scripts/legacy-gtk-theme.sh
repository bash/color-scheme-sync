#!/bin/sh

if [ "$COLOR_SCHEME" = "prefer-dark" ]; then
    gtk_theme=Adwaita-dark
else
    gtk_theme=Adwaita
fi

gsettings set org.gnome.desktop.interface gtk-theme "$gtk_theme"
