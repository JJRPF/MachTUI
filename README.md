# MachTUI: The AI-Native Terminal Engine

MachTUI is a high-performance, next-generation TUI engine for Rust. Designed from the ground up for both human users and AI agents, it combines the power of sub-pixel graphics, CSS-like styling, and strict MVU state management.

## 🚀 Key Features

- **Mach Core:** Immediate-mode rendering with a double-buffered diffing engine for maximum performance.
- **Talon State:** Strict **Model-View-Update (MVU)** architecture inspired by Elm and Bubble Tea.
- **Plume Stylist:** Advanced **MTSS (MachTUI Style Sheets)** parser and Flexbox-inspired layout engine.
- **Vision Layer:** **Sub-pixel rendering** using Braille Unicode characters for high-fidelity terminal graphics.
- **Oracle Protocol:** Built-in **AI-Semantic Tree** generation. Allows LLM agents to "see" and interact with your TUI via structured JSON-RPC.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
machtui = { git = "https://github.com/JJRPF/MachTUI" }
```

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
