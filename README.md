# Santa Racer (Rust Version)

[![latest release](https://badgen.net/github/release/valentjn/santa-racer-rust/stable)](https://github.com/valentjn/santa-racer-rust/releases)&nbsp;
[![CI](https://github.com/valentjn/santa-racer-rust/workflows/CI/badge.svg?branch=develop)](https://github.com/valentjn/santa-racer-rust/actions?query=workflow%3ACI+branch%3Adevelop)&nbsp;
[![license: MPL-2.0](https://badgen.net/github/license/valentjn/santa-racer-rust)](https://github.com/valentjn/santa-racer-rust/blob/develop/LICENSE.md)

Santa Racer is an open-source clone of “Nikolaus Express 2000,” a Christmas-themed advergame/screensaver by German advertising agency Anders und Seim Neue Medien AG. The goal is to steer Santa's sleigh over the roofs of a snowy village and to drop presents into chimneys to score points.

The source code has been written entirely from scratch and is placed under MPL-2.0 (the license only applies to the code, not to the assets).

Before running Santa Racer, the assets must be extracted from the original game (or created from scratch). The original game can only be distributed with the setup EXE as a whole. According to the message box that opens when running the EXE, non-commercial redistribution of the EXE is permitted. However, extracting the assets may be illegal in your jurisdiction (distributing them definitely is). Do this at your own risk!

## Running the Game

### Requirements

- 64-bit Linux or Windows
- SDL 2.0.12 or later (already included in releases)
- SDL_image 2.0.5 or later (already included in releases)
- SDL_mixer 2.0.4 or later (already included in releases)

### How to Run

- Linux: Run `export "LD_LIBRARY_PATH=./:$LD_LIBRARY_PATH" && ./santa-racer`
- Windows: Run `santa-racer.exe`

## Building the Game

### Requirements

- 64-bit Windows or Linux
- Rust 1.47.0 or later
- SDL-devel 2.0.12 or later
- SDL_image-devel 2.0.5 or later
- SDL_mixer-devel 2.0.4 or later
- On Windows: Python 3.6.0 or later
- On Windows: Windows SDK and MSVC Build Tools v142 or later

### Installation of the Requirements

If you're stuck while installing the requirements, a look at [.github/workflows/ci.yml](https://github.com/valentjn/santa-racer-rust/blob/develop/.github/workflows/ci.yml) might help.

#### Linux

- Rust:
  1. Install rustup, either via package (`rustup` on Arch) or via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  2. If the Rust toolchain hasn't already been installed by the previous step, run `rustup update stable`
- SDL-devel, SDL_image-devel, SDL_mixer-devel: Via packages
  - Ubuntu: `libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev`
  - Arch: `sdl2 sdl2_image sdl2_mixer`

#### Windows

- Rust: [Download](https://rustup.rs/) and run `rustup-init.exe`
- Python: [Download](https://www.python.org/) and run `python-*-amd64.exe`
- Windows SDK and MSVC Build Tools:
  1. [Download](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16) and run `vs_BuildTools.exe`
  2. Clear the default selection and only select the Windows SDK (e.g., `Windows 10 SDK`) and the MSVC Build Tools (e.g., `MSVC v142 - VS 2019 C++ x64/x86 build tools`)
- SDL-devel, SDL_image-devel, SDL_mixer-devel: Run `python tools/installSdlForWindows.py`

### How to Build and Run

- `cargo run` (debug mode) or `cargo run --release` (release mode)
