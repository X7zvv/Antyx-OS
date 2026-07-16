# Security Policy

## Grundregeln

- Der private Cosign-Schlüssel darf niemals im Repository, in einer ZIP-Datei, in einem Screenshot oder in einem Chat veröffentlicht werden.
- Der Schlüssel gehört ausschließlich in das GitHub-Secret `SIGNING_SECRET`.
- `cosign.pub` ist öffentlich und darf im Repository liegen.
- SELinux darf nicht deaktiviert werden.
- Die Firewall darf nicht dauerhaft deaktiviert werden.
- Neue Pakete, Flatpaks und Skripte müssen vor Aufnahme geprüft werden.
- Builds müssen aus nachvollziehbarem Quellcode entstehen.
- Keine fremden „FPS Boost“-Skripte ohne vollständige Prüfung übernehmen.

## Sicherheitslücken melden

Bis eine eigene Kontaktadresse existiert, Sicherheitsprobleme nicht öffentlich mit sensiblen Details posten. Nutze zunächst eine private Kontaktmöglichkeit des Projektbetreibers.

## Ehrliche Grenze

Eine eigene Distribution ist nicht automatisch sicherer als Bazzite. Antyx-OS bleibt nur sicher, wenn Updates regelmäßig gebaut, getestet und installiert werden und der Signierschlüssel geschützt bleibt.
