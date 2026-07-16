# Antyx-OS

**Antyx-OS Secure Gaming** is a custom Fedora Atomic gaming operating system with KDE Plasma and an Antyx-owned desktop experience.

The system currently uses the Bazzite NVIDIA gaming image as its technical upstream foundation. Bazzite services and gaming components remain internally where required, while Antyx-OS provides its own visible branding, desktop integration, defaults and control center.

## Alpha 0.4 goals

- KDE Plasma
- modern NVIDIA GPU support
- Antyx boot, login, wallpaper and desktop branding
- native Antyx Hub integration
- Steam preinstalled through the gaming base
- Brave, Discord, Spotify and Modrinth provisioned as system Flatpaks
- SELinux and firewall remain enabled
- atomic updates and rollback
- no visible Bazzite launchers, wallpaper choices or welcome text

## Included applications

- Steam
- Brave Browser
- Discord
- Spotify
- Modrinth App
- Heroic Games Launcher
- Lutris
- ProtonPlus
- OBS Studio
- Flatseal
- Mission Center
- VLC

## Custom commands

```bash
antyx-check
antyx-update
antyx-info
antyx-verify
```

## Project status

Antyx-OS 0.4 is an alpha release. It is not production-ready until installation, updates, rollback, recovery and hardware compatibility have been tested on multiple systems.

## Roadmap

- [x] Build system
- [x] Image signing
- [x] Base applications
- [x] Security defaults
- [x] Logo and visual identity
- [x] KDE colors and wallpaper
- [x] Login theme
- [x] English website
- [x] Antyx Hub
- [x] Alpha 0.4 ISO workflow
- [ ] Installer branding validation
- [ ] Multi-device installation testing
- [ ] Stable 1.0 release
