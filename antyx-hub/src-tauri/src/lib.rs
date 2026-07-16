use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    process::Command,
    thread,
    time::Duration,
};

#[derive(Serialize)]
struct SystemSnapshot {
    cpu_usage: f64,
    cpu_name: String,
    gpu_usage: f64,
    gpu_name: String,
    memory_total_kib: u64,
    memory_used_kib: u64,
    disk_usage_percent: f64,
    disk_used: String,
    disk_total: String,
    os_name: String,
    kernel: String,
    desktop: String,
    uptime: String,
    apps: HashMap<String, bool>,
}

#[derive(Serialize, Clone)]
struct Check {
    level: String,
    value: String,
    description: String,
}

#[derive(Serialize)]
struct SecurityStatus {
    score: u8,
    firewall: Check,
    selinux: Check,
    secure_boot: Check,
    encryption: Check,
    ssh: Check,
    signed_image: Check,
}

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn file_value(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn parse_cpu_total_idle() -> Option<(u64, u64)> {
    let stat = fs::read_to_string("/proc/stat").ok()?;
    let line = stat.lines().next()?;
    let values: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|value| value.parse().ok())
        .collect();
    if values.len() < 5 {
        return None;
    }
    let idle = values[3] + values.get(4).copied().unwrap_or(0);
    let total = values.iter().sum();
    Some((total, idle))
}

fn cpu_usage() -> f64 {
    let first = parse_cpu_total_idle();
    thread::sleep(Duration::from_millis(180));
    let second = parse_cpu_total_idle();
    match (first, second) {
        (Some((total_a, idle_a)), Some((total_b, idle_b))) if total_b > total_a => {
            let delta_total = total_b - total_a;
            let delta_idle = idle_b.saturating_sub(idle_a);
            ((delta_total.saturating_sub(delta_idle)) as f64 / delta_total as f64) * 100.0
        }
        _ => 0.0,
    }
}

fn cpu_name() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|content| {
            content.lines().find_map(|line| {
                line.strip_prefix("model name")
                    .and_then(|rest| rest.split_once(':'))
                    .map(|(_, value)| value.trim().to_string())
            })
        })
        .unwrap_or_else(|| "Unknown CPU".into())
}

fn memory_info() -> (u64, u64) {
    let content = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total = 0;
    let mut available = 0;
    for line in content.lines() {
        if let Some(value) = line.strip_prefix("MemTotal:") {
            total = value.split_whitespace().next().and_then(|v| v.parse().ok()).unwrap_or(0);
        }
        if let Some(value) = line.strip_prefix("MemAvailable:") {
            available = value.split_whitespace().next().and_then(|v| v.parse().ok()).unwrap_or(0);
        }
    }
    (total, total.saturating_sub(available))
}

fn disk_info() -> (f64, String, String) {
    let output = command_output("df", &["-hP", "/"]).unwrap_or_default();
    let line = output.lines().nth(1).unwrap_or_default();
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 5 {
        let percent = parts[4].trim_end_matches('%').parse().unwrap_or(0.0);
        return (percent, parts[2].to_string(), parts[1].to_string());
    }
    (0.0, "Unknown".into(), "Unknown".into())
}

fn gpu_info() -> (f64, String) {
    if let Some(output) = command_output(
        "nvidia-smi",
        &["--query-gpu=utilization.gpu,name", "--format=csv,noheader,nounits"],
    ) {
        if let Some(line) = output.lines().next() {
            if let Some((usage, name)) = line.split_once(',') {
                return (
                    usage.trim().parse().unwrap_or(0.0),
                    name.trim().to_string(),
                );
            }
        }
    }

    let name = command_output("sh", &["-c", "lspci | grep -Ei 'VGA|3D' | head -1"])
        .unwrap_or_else(|| "GPU information unavailable".into());
    (0.0, name)
}

fn os_name() -> String {
    fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| {
            content.lines().find_map(|line| {
                line.strip_prefix("PRETTY_NAME=")
                    .map(|value| value.trim_matches('"').to_string())
            })
        })
        .unwrap_or_else(|| "Antyx-OS".into())
}

fn app_exists(command: &str, flatpak_id: Option<&str>) -> bool {
    let binary = Command::new("sh")
        .args(["-c", &format!("command -v {} >/dev/null 2>&1", command)])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if binary {
        return true;
    }
    flatpak_id
        .and_then(|id| command_output("flatpak", &["info", id]))
        .is_some()
}

#[tauri::command]
fn get_system_snapshot() -> SystemSnapshot {
    let (memory_total_kib, memory_used_kib) = memory_info();
    let (disk_usage_percent, disk_used, disk_total) = disk_info();
    let (gpu_usage, gpu_name) = gpu_info();

    let mut apps = HashMap::new();
    apps.insert("steam".into(), app_exists("steam", Some("com.valvesoftware.Steam")));
    apps.insert("heroic".into(), app_exists("heroic", Some("com.heroicgameslauncher.hgl")));
    apps.insert("lutris".into(), app_exists("lutris", Some("net.lutris.Lutris")));
    apps.insert("protonplus".into(), app_exists("protonplus", Some("com.vysp3r.ProtonPlus")));
    apps.insert("flatseal".into(), app_exists("flatseal", Some("com.github.tchx84.Flatseal")));

    SystemSnapshot {
        cpu_usage: cpu_usage(),
        cpu_name: cpu_name(),
        gpu_usage,
        gpu_name,
        memory_total_kib,
        memory_used_kib,
        disk_usage_percent,
        disk_used,
        disk_total,
        os_name: os_name(),
        kernel: command_output("uname", &["-r"]).unwrap_or_else(|| "Unknown".into()),
        desktop: std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "KDE Plasma".into()),
        uptime: command_output("uptime", &["-p"]).unwrap_or_else(|| "Unknown".into()),
        apps,
    }
}

fn good(value: &str, description: &str) -> Check {
    Check { level: "good".into(), value: value.into(), description: description.into() }
}
fn warning(value: &str, description: &str) -> Check {
    Check { level: "warning".into(), value: value.into(), description: description.into() }
}
fn bad(value: &str, description: &str) -> Check {
    Check { level: "bad".into(), value: value.into(), description: description.into() }
}

#[tauri::command]
fn get_security_status() -> SecurityStatus {
    let firewall = match command_output("systemctl", &["is-active", "firewalld"]).as_deref() {
        Some("active") => good("Enabled", "firewalld is actively filtering incoming network traffic."),
        _ => bad("Disabled", "The firewall is not active."),
    };

    let selinux = match command_output("getenforce", &[]).as_deref() {
        Some("Enforcing") => good("Enforcing", "SELinux mandatory access control is active."),
        Some(value) => warning(value, "SELinux is present but not enforcing policy."),
        None => bad("Unavailable", "SELinux status could not be read."),
    };

    let secure_boot_raw = command_output("mokutil", &["--sb-state"]).unwrap_or_default();
    let secure_boot = if secure_boot_raw.to_lowercase().contains("enabled") {
        good("Enabled", "UEFI Secure Boot is enabled.")
    } else if secure_boot_raw.is_empty() {
        warning("Unknown", "Secure Boot status could not be checked.")
    } else {
        warning("Disabled", "Secure Boot is currently disabled.")
    };

    let root_source = command_output("findmnt", &["-no", "SOURCE", "/"]).unwrap_or_default();
    let encryption = if root_source.contains("crypt") || root_source.contains("mapper") {
        good("Detected", "The root filesystem appears to use block-device encryption.")
    } else {
        warning("Not confirmed", "Disk encryption could not be confirmed from the root mount.")
    };

    let ssh_enabled = command_output("systemctl", &["is-enabled", "sshd"]).unwrap_or_default();
    let ssh = if ssh_enabled == "enabled" {
        warning("Enabled", "The SSH server starts automatically. Disable it when not required.")
    } else {
        good("Disabled", "The SSH server is not enabled by default.")
    };

    let ostree = command_output("rpm-ostree", &["status"]).unwrap_or_default();
    let signed_image = if ostree.contains("ostree-image-signed") || ostree.contains("antyx-os") {
        good("Verified path", "The system is using the expected Atomic image deployment path.")
    } else {
        warning("Review", "The signed Antyx image path could not be confirmed.")
    };

    let checks = [&firewall, &selinux, &secure_boot, &encryption, &ssh, &signed_image];
    let score: i32 = checks.iter().map(|check| match check.level.as_str() {
        "good" => 17,
        "warning" => 9,
        _ => 0,
    }).sum();

    SecurityStatus {
        score: score.min(100) as u8,
        firewall,
        selinux,
        secure_boot,
        encryption,
        ssh,
        signed_image,
    }
}

#[tauri::command]
fn get_fastfetch() -> String {
    command_output("fastfetch", &["--pipe"])
        .or_else(|| command_output("fastfetch", &[]))
        .unwrap_or_else(|| {
            format!(
                "Antyx-OS\nOS: {}\nKernel: {}\nCPU: {}\n",
                os_name(),
                command_output("uname", &["-r"]).unwrap_or_else(|| "Unknown".into()),
                cpu_name()
            )
        })
}

fn spawn_detached(program: &str, args: &[&str]) -> Result<(), String> {
    Command::new(program)
        .args(args)
        .spawn()
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn launch_flatpak_or_binary(binary: &str, flatpak_id: &str) -> Result<(), String> {
    if Command::new("sh")
        .args(["-c", &format!("command -v {} >/dev/null 2>&1", binary)])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        return spawn_detached(binary, &[]);
    }
    spawn_detached("flatpak", &["run", flatpak_id])
}

#[tauri::command]
fn run_action(action: String) -> Result<String, String> {
    match action.as_str() {
        "steam" => {
            launch_flatpak_or_binary("steam", "com.valvesoftware.Steam")?;
            Ok("Steam launched".into())
        }
        "heroic" => {
            launch_flatpak_or_binary("heroic", "com.heroicgameslauncher.hgl")?;
            Ok("Heroic Games Launcher launched".into())
        }
        "lutris" => {
            launch_flatpak_or_binary("lutris", "net.lutris.Lutris")?;
            Ok("Lutris launched".into())
        }
        "protonplus" => {
            launch_flatpak_or_binary("protonplus", "com.vysp3r.ProtonPlus")?;
            Ok("ProtonPlus launched".into())
        }
        "flatseal" => {
            launch_flatpak_or_binary("flatseal", "com.github.tchx84.Flatseal")?;
            Ok("Flatseal launched".into())
        }
        "update" => {
            let terminal_candidates: &[(&str, &[&str])] = &[
                ("konsole", &["-e", "bash", "-lc", "antyx-update; echo; read -p 'Press Enter to close...'"]),
                ("kgx", &["bash", "-lc", "antyx-update; echo; read -p 'Press Enter to close...'"]),
                ("xterm", &["-e", "bash", "-lc", "antyx-update; echo; read -p 'Press Enter to close...'"]),
            ];
            for (program, args) in terminal_candidates {
                if spawn_detached(program, args).is_ok() {
                    return Ok("System update opened in a terminal".into());
                }
            }
            Err("No supported terminal emulator was found".into())
        }
        _ => Err("Unsupported action".into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_system_snapshot,
            get_security_status,
            get_fastfetch,
            run_action
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Antyx Hub");
}
