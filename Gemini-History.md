# Gemini CLI Session History - MachTUI Engine

## Session Context
- **Date:** Sunday, March 22, 2026
- **Project Name:** MachTUI (formerly HawkTUI)
- **Objective:** Design and implement a next-generation TUI engine.

## Research & Decisions
1. **Engine Selection:** Researched the TUI landscape (2025-2026). Selected a hybrid approach inspired by:
   - **Ratatui (Rust):** For high-performance rendering.
   - **Bubble Tea (Go):** For the Model-View-Update (MVU) state architecture.
   - **Textual (Python):** For CSS-based styling (`.mtss` files).
   - **Notcurses (C++):** For advanced terminal graphics/image protocols.
2. **Name Change:** The project was renamed from `HawkTUI` to `MachTUI` to avoid name collisions and better align with the performance focus.
3. **AI-Native Design:** Added the "Oracle" protocol—a semantic tree generator and headless JSON-RPC mode—to ensure AI agents can easily interact with and develop for MachTUI applications.

## Current Progress
- **Project Architecture:** Defined the five core pillars (Mach Core, Talon State, Plume Stylist, Vision Layer, Oracle Protocol).
- **Project Skeleton:** Initialized the Rust library structure in `src/` with placeholders for all architectural modules.
- **Dependencies:** Initial `Cargo.toml` created with `crossterm` as the primary backend.
- **Mach Core:** Implemented the `crossterm` backend and the basic rendering loop in `src/core/`.
- **Talon State:** Fleshed out the `Model` and `Message` traits and implemented the `Program` dispatcher in `src/talon/`.
- **Plume Stylist:** Started the `.mtss` CSS parser with support for simple rule parsing in `src/plume/`.
- **Verification:** Unit tests added for `talon` and `plume`, and confirmed passing.

## Next Steps for Future Session
- Expand the `Renderer` in `src/core/` to support a component-based drawing API.
- Implement a more robust MTSS parser in `src/plume/` using a tokenizer (e.g., `logos` or manual).
- Design the `Vision` layer for sub-pixel rendering and graphics support.
- Integrate `Oracle` for semantic tree generation from the UI model.
