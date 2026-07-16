# Security Policy

## Core rules

- Never store the private signing key in the repository, ZIP archives, screenshots, or chat messages.
- The private key belongs only in the GitHub secret named `SIGNING_SECRET`.
- The public `cosign.pub` file may remain in the repository.
- SELinux must remain enabled.
- The firewall must not be permanently disabled.
- New packages, Flatpaks, scripts, and repositories must be reviewed before inclusion.
- Builds must remain reproducible and traceable.
- Do not include unknown performance scripts without reviewing every command.

## Honest limitation

A custom distribution is not automatically safer than Bazzite. Antyx-OS only remains secure when updates are built, tested, signed, and installed regularly, and when the signing key remains protected.
