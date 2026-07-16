# Image Signing

BlueBuild uses two keys:

- `cosign.pub`: public verification key stored in the repository
- private signing key: stored only as the GitHub secret `SIGNING_SECRET`

GitHub Actions uses the private key to sign every Antyx-OS image. Installed systems use the public key to verify that updates were produced by the expected repository and were not modified.

## Never

- commit the private key
- send the private key to anyone
- show it in screenshots
- print it in workflow logs
- grant repository administrator access to untrusted people

## Key loss

A lost private key cannot be recovered. A new key pair and a controlled trust migration would be required.
