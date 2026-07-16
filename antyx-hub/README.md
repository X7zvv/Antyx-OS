# Antyx Hub v0.1

Antyx Hub is the native control center for Antyx-OS.

## Current features

- live CPU, memory, storage, and NVIDIA GPU usage
- system identity card inspired by Fastfetch
- firewall, SELinux, Secure Boot, disk encryption, SSH, and signed-image status
- security score calculated from real checks
- installed gaming application detection
- quick actions for Steam, Heroic, Lutris, ProtonPlus, and Flatseal
- system update action
- full Fastfetch output
- local recent activity log
- English interface
- no telemetry
- no arbitrary shell execution from the frontend

## Architecture

- Tauri 2
- Rust backend
- plain HTML, CSS, and JavaScript frontend
- fixed command allowlist in Rust
- no Node-based frontend build step

## Development requirements

Install the required tools on Bazzite:

```bash
brew install rustup-init pkg-config
rustup-init -y
source "$HOME/.cargo/env"
cargo install tauri-cli --version "^2.0.0" --locked
```

Bazzite should already provide most desktop runtime libraries. If compilation reports missing WebKitGTK or development libraries, build the application inside a Fedora toolbox:

```bash
toolbox create antyx-dev
toolbox enter antyx-dev
sudo dnf install -y \
  gcc gcc-c++ make pkgconf-pkg-config \
  webkit2gtk4.1-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel openssl-devel
```

Install Rust inside the toolbox if required:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
cargo install tauri-cli --version "^2.0.0" --locked
```

## Run in development mode

```bash
cd Antyx-Hub-v0.1
cargo tauri dev
```

## Build

```bash
cargo tauri build
```

The generated Linux packages are placed under:

```text
src-tauri/target/release/bundle/
```

## Security model

The frontend cannot submit arbitrary commands. It may only invoke predefined Rust functions. Privileged operations are deliberately limited. The update button opens a terminal and runs the existing trusted `antyx-update` command so the user can see and approve password prompts.

## Next versions

- v0.2: update details, notifications, power profile controls
- v0.3: gaming profiles and MangoHud controls
- v0.4: network diagnostics
- v0.5: rollback and recovery interface
