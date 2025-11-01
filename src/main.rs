use anyhow::{Context, Result};
use notify_rust::Notification;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn get_active_wifi_ssid() -> Result<Option<String>> {
    let out = Command::new("nmcli")
        .args(["-t", "-f", "TYPE,NAME", "connection", "show", "--active"])
        .output()
        .context("failed to run nmcli")?;

    if !out.status.success() {
        return Ok(None);
    }

    let s = String::from_utf8_lossy(&out.stdout);
    for line in s.lines() {
        if let Some(idx) = line.find(':') {
            let ty = &line[..idx];
            let name = &line[idx + 1..];
            if ty == "wifi" || ty == "802-11-wireless" {
                return Ok(Some(name.trim().to_string()));
            }
        }
    }

    Ok(None)
}

fn send_notification(summary: &str, body: &str) {
    let _ = Notification::new().summary(summary).body(body).show();
}

fn main() -> Result<()> {
    let interval = Duration::from_secs(3);
    println!("nm-notify: monitoring active Wi-Fi (press Ctrl+C to stop)");

    let mut last_ssid: Option<String> = None;

    loop {
        match get_active_wifi_ssid() {
            Ok(current) => {
                if current != last_ssid {
                    match &current {
                        Some(ssid) => {
                            let msg = format!("Connected to: {ssid}");
                            println!("{}", msg);
                            send_notification("💡 Wi-Fi switched", &msg);
                        }
                        None => {
                            println!("🛑 No active Wi-Fi");
                            send_notification(
                                "🛑 Wi-Fi disconnected",
                                "No active Wi-Fi connection",
                            );
                        }
                    }
                    last_ssid = current;
                }
            }
            Err(e) => {
                eprintln!("Error checking Wi-Fi status: {:#}", e);
                send_notification("Wi-Fi monitor error", &format!("{:#}", e));
            }
        }

        thread::sleep(interval);
    }
}
