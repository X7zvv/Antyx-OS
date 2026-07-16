const invoke = window.__TAURI__.core.invoke;

const state = {
  snapshot: null,
  security: null,
  fastfetch: "",
  activity: JSON.parse(localStorage.getItem("antyx-activity") || "[]")
};

const pages = document.querySelectorAll(".page");
const navItems = document.querySelectorAll(".nav-item");
const title = document.getElementById("page-title");

function showPage(id) {
  pages.forEach(page => page.classList.toggle("active", page.id === id));
  navItems.forEach(item => item.classList.toggle("active", item.dataset.page === id));
  title.textContent = id.charAt(0).toUpperCase() + id.slice(1);
}

navItems.forEach(item => item.addEventListener("click", () => showPage(item.dataset.page)));
document.querySelectorAll(".jump-page").forEach(button =>
  button.addEventListener("click", () => showPage(button.dataset.target))
);

function clamp(value) {
  return Math.max(0, Math.min(100, Number(value) || 0));
}

function updateMetric(name, value, detail) {
  const percent = Math.round(clamp(value));
  document.getElementById(`${name}-value`).textContent = `${percent}%`;
  document.getElementById(`${name}-bar`).style.width = `${percent}%`;
  if (detail) document.getElementById(`${name}-detail`)?.replaceChildren(detail);
}

function formatGiB(kib) {
  return `${(kib / 1024 / 1024).toFixed(1)} GiB`;
}

function addActivity(message) {
  state.activity.unshift({ message, time: new Date().toISOString() });
  state.activity = state.activity.slice(0, 20);
  localStorage.setItem("antyx-activity", JSON.stringify(state.activity));
  renderActivity();
}

function renderActivity() {
  const list = document.getElementById("activity-list");
  if (!state.activity.length) {
    list.innerHTML = '<p class="empty">No local activity yet.</p>';
    return;
  }
  list.innerHTML = state.activity.slice(0, 6).map(item => {
    const date = new Date(item.time);
    return `<div class="activity-item"><i></i><span>${escapeHtml(item.message)}</span><time>${date.toLocaleTimeString([], {hour:"2-digit", minute:"2-digit"})}</time></div>`;
  }).join("");
}

function escapeHtml(value) {
  return String(value).replace(/[&<>"']/g, char => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#039;"
  })[char]);
}

function toast(message) {
  const node = document.createElement("div");
  node.className = "toast";
  node.textContent = message;
  document.getElementById("toast-container").appendChild(node);
  setTimeout(() => node.remove(), 3200);
}

function renderSnapshot(data) {
  state.snapshot = data;
  updateMetric("cpu", data.cpu_usage);
  document.getElementById("cpu-name").textContent = data.cpu_name || "Unknown CPU";

  updateMetric("gpu", data.gpu_usage);
  document.getElementById("gpu-name").textContent = data.gpu_name || "GPU usage unavailable";

  const ramPercent = data.memory_total_kib
    ? data.memory_used_kib / data.memory_total_kib * 100 : 0;
  updateMetric("ram", ramPercent, document.createTextNode(
    `${formatGiB(data.memory_used_kib)} / ${formatGiB(data.memory_total_kib)}`
  ));

  updateMetric("disk", data.disk_usage_percent, document.createTextNode(
    `${data.disk_used} / ${data.disk_total}`
  ));

  const facts = [
    ["OS", data.os_name],
    ["Kernel", data.kernel],
    ["Desktop", data.desktop],
    ["CPU", data.cpu_name],
    ["GPU", data.gpu_name],
    ["Memory", formatGiB(data.memory_total_kib)],
    ["Uptime", data.uptime]
  ];
  document.getElementById("system-facts").innerHTML = facts.map(([key, value]) =>
    `<div><dt>${escapeHtml(key)}</dt><dd title="${escapeHtml(value)}">${escapeHtml(value)}</dd></div>`
  ).join("");

  renderGaming(data.apps);
}

function statusClass(status) {
  if (status === "good") return "ok";
  if (status === "warning") return "warn";
  return "bad";
}

function renderSecurity(data) {
  state.security = data;
  const checks = [
    ["Firewall", data.firewall],
    ["SELinux", data.selinux],
    ["Secure Boot", data.secure_boot],
    ["Disk encryption", data.encryption],
    ["SSH server", data.ssh],
    ["Signed image", data.signed_image]
  ];

  const listHtml = checks.map(([name, item]) =>
    `<div><i class="${statusClass(item.level)}"></i><span>${escapeHtml(name)}</span><b>${escapeHtml(item.value)}</b></div>`
  ).join("");
  document.getElementById("security-list").innerHTML = listHtml;

  document.getElementById("security-details").innerHTML = checks.map(([name, item]) =>
    `<article class="detail-card ${statusClass(item.level)}">
      <span class="mini-label">${escapeHtml(name.toUpperCase())}</span>
      <div class="detail-state">${escapeHtml(item.value)}</div>
      <p>${escapeHtml(item.description)}</p>
    </article>`
  ).join("");

  const score = data.score;
  document.getElementById("security-score").textContent = score;
  document.getElementById("score-ring").style.background =
    `radial-gradient(circle at center, #15111d 62%, transparent 63%), conic-gradient(var(--purple) 0 ${score * 3.6}deg, rgba(255,255,255,.07) ${score * 3.6}deg)`;

  const healthy = score >= 80;
  document.getElementById("welcome-heading").textContent =
    healthy ? "Your system is healthy." : "Your system needs attention.";
  document.getElementById("welcome-text").textContent =
    healthy ? "Core protection is enabled and Antyx-OS is running normally."
            : "Open Security to review checks that require attention.";

  const chip = document.querySelector(".status-chip");
  chip.classList.toggle("healthy", healthy);
  chip.classList.toggle("warning", !healthy);
  document.getElementById("system-state").textContent =
    healthy ? "System healthy" : "Review security";
}

function renderGaming(apps) {
  const definitions = [
    ["steam", "Steam", "◉", "Game library and Proton"],
    ["heroic", "Heroic Games Launcher", "H", "Epic, GOG, and Amazon games"],
    ["lutris", "Lutris", "L", "Linux gaming manager"],
    ["protonplus", "ProtonPlus", "P", "Manage compatibility tools"],
    ["flatseal", "Flatseal", "⬡", "Review Flatpak permissions"]
  ];

  document.getElementById("gaming-apps").innerHTML = definitions.map(([id, name, icon, description]) => {
    const installed = Boolean(apps[id]);
    return `<article class="app-card">
      <div class="app-icon">${icon}</div>
      <h3>${name}</h3>
      <p>${installed ? description : "Not detected on this system"}</p>
      <button data-action="${id}" ${installed ? "" : "disabled"}>${installed ? "Launch" : "Not installed"}</button>
    </article>`;
  }).join("");

  bindActionButtons();
}

function bindActionButtons() {
  document.querySelectorAll("[data-action]").forEach(button => {
    if (button.dataset.bound) return;
    button.dataset.bound = "true";
    button.addEventListener("click", async () => {
      const action = button.dataset.action;
      button.disabled = true;
      try {
        const message = await invoke("run_action", { action });
        toast(message);
        addActivity(message);
      } catch (error) {
        toast(`Action failed: ${error}`);
      } finally {
        button.disabled = false;
      }
    });
  });
}

async function refreshAll(log = false) {
  const refresh = document.getElementById("refresh-button");
  refresh.disabled = true;
  try {
    const [snapshot, security, fastfetch] = await Promise.all([
      invoke("get_system_snapshot"),
      invoke("get_security_status"),
      invoke("get_fastfetch")
    ]);
    renderSnapshot(snapshot);
    renderSecurity(security);
    state.fastfetch = fastfetch;
    document.getElementById("fastfetch-output").textContent = fastfetch;
    if (log) addActivity("System information refreshed");
  } catch (error) {
    toast(`Unable to read system information: ${error}`);
  } finally {
    refresh.disabled = false;
  }
}

document.getElementById("refresh-button").addEventListener("click", () => refreshAll(true));
document.getElementById("full-fastfetch").addEventListener("click", () => showPage("system"));
document.getElementById("clear-activity").addEventListener("click", () => {
  state.activity = [];
  localStorage.removeItem("antyx-activity");
  renderActivity();
});
document.getElementById("copy-fastfetch").addEventListener("click", async () => {
  await navigator.clipboard.writeText(state.fastfetch || "");
  toast("System report copied");
  addActivity("Fastfetch report copied");
});

renderActivity();
bindActionButtons();
refreshAll();
setInterval(() => refreshAll(false), 5000);
