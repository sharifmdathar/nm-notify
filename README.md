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

### Download Pre-compiled Binary

If you don't want to compile from source, you can download a pre-compiled binary from the [releases page](https://github.com/sharifmdathar/nm-notify/releases).

### Build from Source

**Note:** Building from source requires **Rust** to be installed.

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

When you move to another Wi-Fi (or NetworkManager roams automatically), you'll get a desktop notification like:

<img width="538" height="87" alt="screenshot-2025-11-01_22-26-42" src="https://github.com/user-attachments/assets/d542c911-135d-4293-bc26-f0b9267571fa" />


## 🔄 Autostart at Login (systemd User Service)

To make `nm-notify` automatically start when you log in, you can set it up as a systemd user service.

### Setup

1. **Copy the binary to `~/.local/bin`** (recommended location for user-installed binaries):

   ```bash
   # Create the directory if it doesn't exist
   mkdir -p ~/.local/bin
   
   # Copy the binary (adjust the source path based on how you obtained it)
   # If you built from source:
   cp target/release/nm-notify ~/.local/bin/nm-notify
   
   # If you downloaded a pre-compiled binary:
   # cp /path/to/downloaded/nm-notify ~/.local/bin/nm-notify
   
   # Make sure it's executable
   chmod +x ~/.local/bin/nm-notify
   ```

   **Note:** Make sure `~/.local/bin` is in your PATH. Most Linux distributions include it by default, but if it's not, add this to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.):
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

2. **Create the systemd user service directory** (if it doesn't exist):

   ```bash
   mkdir -p ~/.config/systemd/user
   ```

3. **Create the service file**:

   ```bash
   nano ~/.config/systemd/user/nm-notify.service
   ```

4. **Add the following content**:

   ```ini
   [Unit]
   Description=NetworkManager Wi-Fi change notifier
   After=network.target NetworkManager.service
   Wants=NetworkManager.service

   [Service]
   Type=simple
   ExecStart=%h/.local/bin/nm-notify
   Restart=on-failure
   RestartSec=5

   [Install]
   WantedBy=default.target
   ```

   **Note:** The `%h` in `ExecStart` is a systemd specifier that expands to your home directory. If you prefer, you can use an absolute path like `/home/username/.local/bin/nm-notify` instead.

5. **Reload systemd and enable the service**:

   ```bash
   systemctl --user daemon-reload
   systemctl --user enable --now nm-notify.service
   ```

6. **Verify it's running**:
   ```bash
   systemctl --user status nm-notify.service
   ```
