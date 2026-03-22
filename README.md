# MachTUI: The AI-Native Terminal Engine

MachTUI is a high-performance, next-generation TUI engine for Rust. Designed from the ground up for both human users and AI agents, it combines the power of sub-pixel graphics, CSS-like styling, and strict MVU state management.

## 🚀 Key Features

- **Mach Core:** Immediate-mode rendering with a double-buffered diffing engine and **built-in FPS tracking**.
- **Component System:** Reusable, modular UI widgets like `ProgressBar` and `BoxComponent`.
- **Talon State:** Strict **Model-View-Update (MVU)** architecture with asynchronous `Cmd` support.
- **Plume Stylist:** Advanced **MTSS (MachTUI Style Sheets)** with support for **ID (#)** and **Class (.)** selectors.
- **Vision Layer:** **Sub-pixel Braille rendering**, **RGB Gradients**, and **ASCII Art** utilities.
- **Oracle Protocol:** **AI-Semantic Tree** generation with rich metadata for seamless LLM interaction.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
machtui = { git = "https://github.com/JJRPF/MachTUI" }
```

## 🛠️ Mach CLI

MachTUI comes with a powerful CLI tool to manage your application lifecycle.

### Installation
If you have the source, install it globally:
```bash
cargo install --path .
```

### Usage
- **`mach config`**: Launch the TUI configurator to manage local/SSH serving and explore examples.
- **`mach new <name>`**: Scaffold a new MachTUI project.
- **`mach run`**: Run your current project.
- **`mach oracle`**: Inspect your UI's semantic tree or start a headless AI server.

## 🎮 Demos

Explore the capabilities of MachTUI with our built-in examples:

### 1. Counter (Human TUI)
Showcases MVU state management and interactive terminal UI.
```bash
cargo run --example counter
```

### 2. Vision Waves (Graphics)
Showcases sub-pixel Braille rendering for smooth animations.
```bash
cargo run --example vision_waves
```

### 3. Oracle Headless (AI Protocol)
Showcases the semantic JSON tree exported for AI consumption.
```bash
cargo run --example oracle_headless
```

## 🏗️ Architecture

MachTUI is built on five pillars:
1. **Mach:** The rendering core.
2. **Talon:** The state management engine.
3. **Plume:** The styling and layout stylist.
4. **Vision:** The graphics and sub-pixel layer.
5. **Oracle:** The AI semantic gateway.

## 📄 License
MIT / Apache 2.0
