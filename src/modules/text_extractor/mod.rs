#![allow(dead_code)]

pub mod model;

use std::path::{Path, PathBuf};
use std::process::Command;

/// Copies text to both Wayland clipboard and primary selection
pub fn copy_to_clipboard(text: &str) {
    if let Ok(mut child) = Command::new("wl-copy")
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
    }

    if let Ok(mut child) = Command::new("wl-copy")
        .arg("-p")
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
    }
}

/// Dispatches desktop notification with icon and PowerToys identity
pub fn notify(summary: &str, body: &str) {
    let _ = Command::new("notify-send")
        .arg("-a")
        .arg("PowerToys")
        .arg("-i")
        .arg("edit-copy")
        .arg(summary)
        .arg(body)
        .spawn();
}

/// Finds the extract.sh script from the Caelestia Plugins directory if present
pub fn find_caelestia_extract_script() -> Option<PathBuf> {
    // 1. Direct project directory
    let dev_project = PathBuf::from("/home/gopik/Projects/Caelestia Plugins/text-extractor/extract.sh");
    if dev_project.is_file() {
        return Some(dev_project);
    }

    // 2. User home config location
    if let Ok(home) = std::env::var("HOME") {
        let config_script = PathBuf::from(&home)
            .join(".config/caelestia/plugins/text-extractor/extract.sh");
        if config_script.is_file() {
            return Some(config_script);
        }

        let home_dev = PathBuf::from(&home)
            .join("Projects/Caelestia Plugins/text-extractor/extract.sh");
        if home_dev.is_file() {
            return Some(home_dev);
        }
    }

    // 3. System launcher in PATH
    if let Ok(output) = Command::new("which").arg("caelestia-text-extractor").output() {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let p = PathBuf::from(path_str);
            if p.is_file() {
                return Some(p);
            }
        }
    }

    None
}

/// Invokes the OCR pipeline (using Caelestia plugin extract.sh when available, or native Wayland fallback)
pub fn extract_text_sync(
    lang: &str,
    psm: u8,
    single_line: bool,
    auto_copy: bool,
    send_notification: bool,
) -> Option<String> {
    // Attempt to invoke Caelestia plugin extract.sh
    if let Some(script_path) = find_caelestia_extract_script() {
        let mut cmd = Command::new(script_path);
        cmd.arg("-l").arg(lang);
        cmd.arg("-p").arg(psm.to_string());

        if single_line {
            cmd.arg("-s");
        } else {
            cmd.arg("-m");
        }

        if !auto_copy {
            cmd.arg("--no-copy");
        }
        if !send_notification {
            cmd.arg("--no-notify");
        }

        if let Ok(output) = cmd.output() {
            if output.status.success() {
                let stdout_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !stdout_str.is_empty() {
                    return Some(stdout_str);
                }
            }
        }
    }

    // Native Wayland Fallback pipeline (slurp + grim + tesseract)
    native_extract_fallback(lang, psm, single_line, auto_copy, send_notification)
}

fn native_extract_fallback(
    lang: &str,
    psm: u8,
    single_line: bool,
    auto_copy: bool,
    send_notification: bool,
) -> Option<String> {
    // 1. Slurp area selection
    let slurp_output = Command::new("slurp")
        .args([
            "-d",
            "-b", "#0000002b",
            "-c", "#89B4FAff",
            "-s", "#89B4FA22",
            "-w", "2",
        ])
        .output()
        .ok()?;

    if !slurp_output.status.success() {
        return None; // Cancelled
    }

    let geometry = String::from_utf8_lossy(&slurp_output.stdout).trim().to_string();
    if geometry.is_empty() {
        return None;
    }

    // 2. Grim capture to temporary file
    let tmp_path = format!("/tmp/powertoys_ocr_{}.png", std::process::id());
    let grim_status = Command::new("grim")
        .args(["-g", &geometry, &tmp_path])
        .status()
        .ok()?;

    if !grim_status.success() || !Path::new(&tmp_path).exists() {
        let _ = std::fs::remove_file(&tmp_path);
        return None;
    }

    // 3. Tesseract execution
    let tesseract_output = Command::new("tesseract")
        .args([
            &tmp_path,
            "stdout",
            "-l", lang,
            "--psm", &psm.to_string(),
        ])
        .output();

    let _ = std::fs::remove_file(&tmp_path);

    let output = tesseract_output.ok()?;
    if !output.status.success() {
        return None;
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let mut cleaned = raw.trim().to_string();

    if single_line {
        cleaned = model::to_single_line(&cleaned);
    }

    if cleaned.is_empty() {
        if send_notification {
            notify("No Text Found", "No readable text detected in the selected area.");
        }
        return None;
    }

    // 4. Clipboard synchronization
    if auto_copy {
        copy_to_clipboard(&cleaned);
    }

    // 5. Desktop notification with preview
    if send_notification {
        let snippet = model::snippet(&cleaned, 80);
        notify("Text Extracted & Copied", &snippet);
    }

    // 6. Notify running Caelestia Shell if present
    let _ = Command::new("qs")
        .args(["-c", "caelestia", "ipc", "call", "textExtractor", "update", &cleaned])
        .spawn();

    Some(cleaned)
}
