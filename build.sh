#!/usr/bin/env sh
chmod 0755 ./appimagetool-x86_64.AppImage
./appimagetool-x86_64.AppImage src/linux/src/appimage/mcchunkget3cli.AppDir src/linux/target/appimage/mcchunkgetcli-x86_64.AppImage &&
chmod 0755 src/linux/target/appimage/mcchunkgetcli-x86_64.AppImage
