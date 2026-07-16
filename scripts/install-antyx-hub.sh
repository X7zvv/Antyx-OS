#!/usr/bin/env bash
set -euo pipefail

RPM="/ctx/packages/antyx-hub/antyx-hub-0.1.0-1.x86_64.rpm"

echo "Installing Antyx Hub..."

rpm-ostree install "$RPM"

mkdir -p /usr/share/applications

cat >/usr/share/applications/antyx-hub.desktop <<EOF
[Desktop Entry]
Name=Antyx Hub
Comment=Antyx-OS Control Center
Exec=env WEBKIT_DISABLE_DMABUF_RENDERER=1 antyx-hub
Icon=utilities-system-monitor
Terminal=false
Type=Application
Categories=System;Settings;
EOF

echo "Antyx Hub installed."
