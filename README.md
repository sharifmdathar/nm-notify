# 🛰️ nm-notify

A lightweight Rust daemon that shows a **desktop notification** whenever NetworkManager automatically switches your Wi-Fi connection.

This is useful if you roam between multiple access points or Wi-Fi networks and want to be informed whenever your system connects to a different SSID.

---

## ✨ Features

- ✅ Detects Wi-Fi changes using `nmcli`
- 💬 Shows desktop notifications via `libnotify`
- 🪶 Lightweight (no polling overhead beyond a 3s check)
- 🔧 Works on all NetworkManager-based Linux distributions

---

## 📦 Installation

### Prerequisites

Make sure you have:

- **NetworkManager** (`nmcli` command available)
- **libnotify** (usually preinstalled on GNOME, KDE, XFCE, etc.)
- **Rust** (to build from source)

### Build

```bash
git clone https://github.com/sharifmdathar/nm-notify.git
cd nm-notify
cargo build --release
```

The compiled binary will be at:

target/release/nm-notify

## 🚀 Usage

Run it manually:

```bash
./target/release/nm-notify
```

You’ll see:

```
nm-notify: monitoring active Wi-Fi (press Ctrl+C to stop)
Connected to: MyHomeNetwork
```

When you move to another Wi-Fi (or NetworkManager roams automatically), you’ll get a desktop notification like:
<img width="532" height="78" alt="image" src="https://github.com/user-attachments/assets/132a68b6-5ecd-4e13-b7bc-fe69cf9da279" />
