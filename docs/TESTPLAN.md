# Testplan für Antyx-OS 0.1

## Build

- GitHub Action läuft ohne Fehler.
- Das Image wird unter GHCR veröffentlicht.
- Die Cosign-Signatur lässt sich prüfen.

## Start

- Das System bootet.
- KDE-Anmeldung funktioniert.
- NVIDIA-Treiber werden mit `nvidia-smi` erkannt.
- Beide Monitore funktionieren.
- Audio, WLAN/LAN, Bluetooth, Tastatur und Maus funktionieren.

## Gaming

- Steam startet.
- Ein natives Spiel startet.
- Ein Proton-Spiel startet.
- MangoHud funktioniert.
- Controller wird erkannt.
- Heroic und Lutris starten.

## Sicherheit

- `getenforce` zeigt `Enforcing`.
- `systemctl is-active firewalld` zeigt `active`.
- Secure Boot bleibt aktiv.
- SSH ist nicht unbeabsichtigt aktiviert.
- Updates stammen aus dem eigenen signierten Image.
- Rollback auf das vorherige Deployment funktioniert.

## Apps

- Brave startet.
- Discord startet.
- OBS startet.
- Flatseal zeigt die Flatpak-Berechtigungen.
