# Antyx-OS

**Antyx-OS Secure Gaming** ist ein eigenes, signiertes Gaming-System auf Basis von Bazzite und Fedora Atomic.

## Ziel der ersten Version

- KDE Plasma
- moderne NVIDIA-Grafikkarten, einschließlich RTX 50-Serie
- Brave Browser vorinstalliert
- Steam und Gaming-Komponenten aus Bazzite
- Heroic, Lutris, ProtonPlus, Discord, OBS, Flatseal, Mission Center und VLC
- SELinux und Firewall bleiben aktiv
- Atomic Updates und Rollback
- ausgewogene Sicherheitsregeln ohne aggressive Gaming-Probleme

## Wichtig

Antyx-OS ist in dieser Phase ein privates Testprojekt. Veröffentliche keine ISO als „sicher“ oder „fertig“, bevor Builds, Updates, Installation und Rollback mehrfach getestet wurden.

## Repository einrichten

1. Öffne den BlueBuild Workshop.
2. Melde dich mit GitHub an.
3. Erstelle ein Repository namens `antyx-os`.
4. Wähle die automatische Einrichtung der Container-Signierung.
5. Lass die vom Workshop erstellte Datei `cosign.pub` im Repository.
6. Kopiere danach die Dateien aus diesem Starterprojekt in das Repository.
7. Ersetze überall `X7zvv` mit deinem echten GitHub-Namen.
8. Committe und pushe die Dateien.
9. Öffne auf GitHub den Reiter **Actions** und starte **Build Antyx-OS**.

## Installation auf einem bestehenden Bazzite-System

Ersetze `X7zvv`:

```bash
sudo rpm-ostree rebase \
  ostree-unverified-registry:ghcr.io/X7zvv/antyx-os:latest

systemctl reboot
```

Nach dem ersten erfolgreichen Start auf das signierte Image wechseln:

```bash
sudo rpm-ostree rebase \
  ostree-image-signed:docker://ghcr.io/X7zvv/antyx-os:latest

systemctl reboot
```

## Rückkehr zu normalem Bazzite

```bash
sudo rpm-ostree rebase \
  ostree-image-signed:docker://ghcr.io/ublue-os/bazzite-nvidia-open:stable

systemctl reboot
```

Bei einem fehlgeschlagenen Update kannst du im Bootmenü das vorherige Deployment starten oder Folgendes verwenden:

```bash
sudo rpm-ostree rollback
systemctl reboot
```

## Eigene Befehle

```bash
antyx-check
antyx-update
antyx-info
```

## Projektstatus

- [x] Technische Grundstruktur
- [x] Automatischer Image-Build
- [x] Brave und Gaming-Apps
- [x] Ausgewogene Sicherheitsbasis
- [x] Erste Website
- [ ] Eigenes Logo
- [ ] Wallpaper und KDE-Branding
- [ ] Eigene Secure-Boot-Schlüssel
- [ ] Installierbare ISO
- [ ] Antyx-Hub
