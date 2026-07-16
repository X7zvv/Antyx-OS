# Antyx-OS Website v1

Eigenständige, responsive Website im cleanen Dashboard-Stil mit weiß-lila Antyx-Branding.

## Enthalten

- moderne Startseite
- animiertes Dashboard-Mockup
- Hover-Spotlight auf Feature-Karten
- Scroll-Reveal-Effekte
- responsive mobile Navigation
- Features, Sicherheit, Roadmap und Download-Bereich
- keine Frameworks und keine externen Abhängigkeiten
- GitHub-Pages-kompatibel

## Lokal testen

```bash
cd Antyx-OS-Website-v1
python3 -m http.server 8080
```

Dann im Browser öffnen:

```text
http://localhost:8080
```

## In dein Antyx-OS-Repository übernehmen

Den Inhalt dieses Ordners nach `website/` kopieren:

```bash
rm -rf ~/Downloads/Antyx-OS/website
cp -a ~/Downloads/Antyx-OS-Website-v1 ~/Downloads/Antyx-OS/website

cd ~/Downloads/Antyx-OS
git add website
git commit -m "feat: redesign Antyx-OS website"
git push origin main
```

## GitHub Pages aktivieren

Im Repository:

1. **Settings**
2. **Pages**
3. Unter **Build and deployment**: `Deploy from a branch`
4. Branch: `main`
5. Ordner: `/website` wird von GitHub Pages nicht direkt unterstützt.

Am saubersten wird die Website später über einen eigenen GitHub-Pages-Workflow bereitgestellt. Alternativ können die Webdateien in einen separaten `antyx-os.github.io`-Repository gelegt werden.

## Anpassen

GitHub-Link in `index.html`:

```html
https://github.com/X7zvv/Antyx-OS
```

Texte, Roadmap und Versionsnummer können direkt in `index.html` geändert werden.
