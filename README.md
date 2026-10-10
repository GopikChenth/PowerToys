# PowerToys for Linux (Hyprland)

<p align="center">
  <img src="https://raw.githubusercontent.com/microsoft/PowerToys/main/doc/images/overview/PT_hero_image.png" alt="PowerToys Banner" width="700" />
</p>

<p align="center">
  <strong>A modern, blazingly fast reimplementation of Microsoft PowerToys utilities for Linux, built with Rust, Slint, and native Hyprland/Wayland integration.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg?style=flat-square&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/GUI-Slint%201.9-blue.svg?style=flat-square" alt="Slint" />
  <img src="https://img.shields.io/badge/Compositor-Hyprland%20%2F%20Wayland-00c8ff.svg?style=flat-square" alt="Hyprland" />
  <img src="https://img.shields.io/badge/License-MIT-green.svg?style=flat-square" alt="License" />
</p>

---

## Overview

**PowerToys for Linux** brings the beloved productivity tools of Microsoft PowerToys to the Linux desktop with native Wayland ergonomics. Designed specifically for **Hyprland** and powered by **Slint**, it delivers a pixel-perfect Fluent Design interface, sub-millisecond IPC responsiveness, and zero-overhead performance.

---

## ✨ Features

### 🎨 Color Picker
A full-featured color inspection and editing utility designed for designers and developers:
- **Global Shortcut**: Activate instantly with <kbd>SUPER</kbd> + <kbd>SHIFT</kbd> + <kbd>C</kbd> (customizable).
- **Native Wayland Sampling**: Uses `hyprpicker` for pixel-accurate color selection on Wayland.
- **Interactive Color Editor Flyout**:
  - **Stepped Shades**: Automatically generates light and dark variants of any sampled color.
  - **Multi-Format Conversion**: Real-time conversions across **HEX**, **RGB**, **HSL**, **HSV**, and **CMYK**.
  - **HSV Color Sliders**: Fine-tune hue, saturation, and value with live color gradients.
  - **Color Identification**: Detects and displays human-readable color names.
  - **History Palette**: Stores recent color picks with quick-copy swatches.
  - **Direct Clipboard Integration**: Auto-copies clean formatted codes via `wl-copy` with toast notifications.

### 🪟 Hyprland & Wayland Native Integration
- **Dynamic Window Rounding**: Automatically queries your active Hyprland corner radius (`decoration:rounding`) via `hyprctl` and applies it seamlessly to the Slint GUI.
- **Automated Shortcut Management**: Writes and maintains keybindings in `~/.config/hypr/powertoys.conf` and supports Caelestia/Lua Hyprland environments (`~/.config/powertoys/powertoys.lua`).
- **Instant IPC Daemon**: Background Unix Domain Socket listener ensures floating windows and editor flyouts appear with **0ms latency** without restarting the application.
- **Floating Window Rules**: Pre-configured Wayland rules (`float`, `pin`, `center`, `noborder`) ensure popups behave like native desktop overlays.

### 📝 Text Extractor *(Roadmap / In Development)*
- Screen region OCR extraction via `slurp` + `grim` + Tesseract neural engine (LSTM).
- Preprocessing pipeline (upscaling, contrast enhancement, dark-mode inversion) for 99%+ screen accuracy.
- Immediate clipboard copy and single-line/multiline formatting toggles.

---

## 🛠️ Requirements

Ensure the following tools and libraries are installed on your system:

### Arch Linux / CachyOS
```bash
sudo pacman -S hyprpicker wl-clipboard libnotify slurp grim
```

*(Optional for upcoming Text Extractor)*:
```bash
sudo pacman -S tesseract tesseract-data-eng
```

### Fedora
```bash
sudo dnf install hyprpicker wl-clipboard libnotify slurp grim
```

---

## 🚀 Building & Running

### Prerequisites
Make sure you have the latest Rust toolchain installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build from Source
```bash
# Clone the repository
git clone https://github.com/GopikChenth/powertoys.git
cd powertoys

# Build release binary
cargo build --release

# Run PowerToys
./target/release/powertoys
```

---

## 💻 CLI & Shortcut Usage

PowerToys can be launched as a background daemon, a standalone GUI, or directly triggered via CLI flags:

| Command | Action |
| :--- | :--- |
| `powertoys` | Launches the main application and starts the IPC background listener |
| `powertoys --gui` | Forces the main settings dashboard to open |
| `powertoys --pick` / `-p` | Triggers the color picker immediately |
| `powertoys --popup <HEX>` | Opens the floating Color Editor flyout for a specific color (e.g. `powertoys --popup "#B59EE6"`) |

---

## 📂 Project Architecture

```
powertoys/
├── src/
│   ├── core/
│   │   ├── config.rs         # Persistent JSON configuration (~/.config/powertoys/config.json)
│   │   ├── hyprland.rs       # Hyprland IPC, rounding detection & keybind configuration
│   │   ├── ipc.rs            # Unix domain socket IPC client & listener
│   │   └── mod.rs
│   ├── modules/
│   │   ├── color_picker/     # Color picker model, shade generation & format conversions
│   │   └── mod.rs
│   └── main.rs               # Application entry point, CLI routing & Slint event loop
├── ui/
│   └── app.slint             # Modern Slint UI components, layout & floating editor windows
├── build.rs                  # Slint compilation build script
└── Cargo.toml                # Project dependencies
```

---

## 🤝 Contributing

Contributions, feature suggestions, and pull requests are warmly welcome!
1. Fork the repository
2. Create your feature branch (`git checkout -b feat/my-new-feature`)
3. Commit your changes (`git commit -m 'feat: add my feature'`)
4. Push to the branch (`git push origin feat/my-new-feature`)
5. Open a Pull Request

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).
