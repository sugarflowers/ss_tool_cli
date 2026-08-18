# 📸 ss_tool_cli — Interval Screenshot Capture Tool

## Overview

**ss_tool_cli** is a lightweight command‑line tool that automatically captures screenshots of your PC at fixed intervals.  
It is useful for activity logging, long‑running monitoring tasks, debugging, and visual record‑keeping.

- Supports **Windows** and **macOS**  
- Saves PNG files with zero‑padded sequential numbering  
- Multi‑display environments supported (each display saved into its own directory)  
- Simple CLI interface with minimal dependencies

---

## ✨ Features

- **Interval-based screen capture**  
  Capture screenshots every *N* seconds.

- **Multi-display support**  
  Each display is saved under its own directory (`0/`, `1/`, …).

- **Sequential filenames**  
  Files are saved as `000000.png`, `000001.png`, making them easy to sort and process.

- **Simple CLI usage**  
  Only two required arguments: `path` and `interval`.

---

## 🔧 Installation

Prebuilt binaries are available via GitHub Actions Artifacts:

- Windows: `windows-app/ss_tool_cli.exe`
- macOS: `macos-app/ss_tool_cli`

Linux builds are currently on hold due to dependency complexity.

---

## 🚀 Usage

### Basic syntax

```
ss_tool_cli <path> <interval>
```

- **path** — Directory where screenshots will be saved  
- **interval** — Capture interval in seconds

### Example: Capture every 5 seconds into `./captures`

```
ss_tool_cli captures 5
```

This produces a directory structure like:

```
captures/
  0/
    000000.png
    000001.png
    ...
  1/
    000000.png
    000001.png
    ...
```

---

## 📁 Save Format

- Directories are created automatically if they do not exist  
- Each display gets its own subdirectory  
- PNG format  
- Filenames are 6‑digit zero‑padded sequential numbers

---

## 🖥️ Multi-display Behavior

Currently, **all displays are captured**.

A future update will introduce a `target` argument:

```
ss_tool_cli captures 5 1   # Capture only display 1
```

This feature is planned but not yet implemented.

---

## ⚠️ Notes

- On macOS, you must grant **Screen Recording permission** on first launch  
- On Windows, some applications may appear black due to UAC or GPU restrictions  
- Linux builds are postponed due to heavy Wayland/X11 dependencies

---

## 🛠️ For Developers

Built with Rust using the `screenshots` crate.

Key components:

- **Capture struct**  
  - Manages paths  
  - Counts existing files  
  - Handles capture logic

- **clap** for CLI argument parsing  
- Platform-specific screenshot handling is abstracted by the `screenshots` crate

---

## 📄 License

MIT License

---

## 🎉 Final Notes

**ss_tool_cli** aims to be a simple, fast, and reliable interval screenshot tool.  
Suggestions, improvements, and pull requests are welcome.
