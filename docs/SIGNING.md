# Signierung verständlich erklärt

Der BlueBuild Workshop erzeugt ein Schlüsselpaar:

- `cosign.pub`: öffentlicher Schlüssel. Er liegt sichtbar im Repository.
- privater Schlüssel: liegt verschlüsselt als GitHub-Secret `SIGNING_SECRET`.

GitHub Actions verwendet den privaten Schlüssel, um jedes gebaute Antyx-OS-Image zu signieren. Das installierte System verwendet den öffentlichen Schlüssel, um zu prüfen, ob das Image wirklich aus deinem Repository stammt und unverändert ist.

## Niemals tun

- privaten Schlüssel in Dateien speichern, die zu GitHub gepusht werden
- privaten Schlüssel verschicken
- privaten Schlüssel in Screenshots zeigen
- das Secret in Workflow-Ausgaben ausgeben
- fremde Personen als Repository-Admin hinzufügen, denen du nicht vollständig vertraust

## Verlust

Geht der private Schlüssel verloren, kann er nicht wiederhergestellt werden. Dann muss ein neues Schlüsselpaar erstellt und die Vertrauenskette sauber umgestellt werden.
