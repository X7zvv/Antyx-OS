# Antyx-OS: erster gemeinsamer Schritt

## Was du jetzt machst

1. Erstelle über den BlueBuild Workshop das Repository `antyx-os`.
2. Aktiviere dort die automatische Signierung.
3. Lade dieses Starterprojekt herunter und entpacke es.
4. Kopiere den Inhalt in das neue GitHub-Repository.
5. Lösche die vom Workshop erstellte `cosign.pub` nicht.
6. Ersetze `X7zvv` im gesamten Projekt.
7. Starte den Build über GitHub Actions.

## Was danach geprüft wird

- Ist der Build grün?
- Existiert das Paket unter `ghcr.io/X7zvv/antyx-os`?
- Startet Brave nach der ersten Anmeldung?
- Funktionieren Steam, NVIDIA und MangoHud?
- Zeigt `antyx-check` SELinux und Firewall korrekt an?
- Funktioniert ein Rollback?

Erst danach bauen wir Logo, Wallpaper, KDE-Theme und ISO.
