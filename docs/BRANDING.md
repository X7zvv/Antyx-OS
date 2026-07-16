# Branding installieren und testen

Die Dateien dieses Pakets sind bereits in `files/system/` einsortiert und werden durch das bestehende BlueBuild-`files`-Modul ins Image kopiert.

## Enthalten

- Antyx-Logo als SVG und PNG
- 4K-Standard-Wallpaper
- KDE-Farbschema `Antyx`
- SDDM-Anmeldetheme
- Plymouth-Boottheme
- Fastfetch-Preset
- automatische Anwendung von Wallpaper und Farbschema beim ersten KDE-Login
- violette Website

## Nach dem Upload

```bash
cd ~/Downloads/Antyx-OS
git add .
git commit -m "feat: add complete Antyx branding"
git push origin main
```

Danach den Build beobachten:

```bash
gh run watch
```

## Test auf dem eigenen Bazzite-System

Zuerst das neue Image installieren/rebasen. Nach dem Neustart:

```bash
antyx-info
fastfetch --config /usr/share/fastfetch/presets/antyx.jsonc
```

Prüfen:

- Wallpaper gesetzt
- Farbschema `Antyx`
- Login-Screen nach Abmelden/Neustart
- Antyx-Logo in `/usr/share/icons`
- Brave, Steam und Gaming-Apps starten

## Plymouth-Hinweis

Das Boottheme liegt im Image und ist als Standard eingetragen. Bei Atomic-Systemen kann für die Aufnahme ins initramfs später noch ein eigener Build-Schritt nötig sein. Das testen wir nach dem nächsten erfolgreichen Image-Build. Das Theme selbst ist bereits fertig.
