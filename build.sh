#!/usr/bin/env sh
chmod +x ./appimagetool-x86_64.AppImage
./appimagetool-x86_64.AppImage src/linux/src/appimage/mcchunkget3cli.AppDir src/linux/target/appimage/mcchunkgetcli-x86_64.AppImage
chmod +x src/linux/target/appimage/mcchunkgetcli-x86_64.AppImage
