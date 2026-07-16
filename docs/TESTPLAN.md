# Antyx-OS Test Plan

## Build

- GitHub Actions completes successfully.
- The image is published to GHCR.
- The image signature can be verified.

## Boot and hardware

- The system boots successfully.
- KDE login works.
- NVIDIA is detected with `nvidia-smi`.
- Displays, audio, network, Bluetooth, keyboard, and mouse work.

## Gaming

- Steam launches.
- A native Linux game launches.
- A Proton game launches.
- MangoHud works.
- Controllers are detected.
- Heroic and Lutris launch.

## Security

- `getenforce` returns `Enforcing`.
- `systemctl is-active firewalld` returns `active`.
- Secure Boot remains enabled.
- SSH is not unintentionally enabled.
- Signed image updates work.
- Rollback works.

## Applications

- Brave launches.
- Discord launches.
- OBS launches.
- Flatseal displays Flatpak permissions.
