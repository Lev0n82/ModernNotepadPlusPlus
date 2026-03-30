---
description: Modern Notepad++ migration to Rust with Null Claw integration
---

# Migration Workflow

This workflow guides the step‑by‑step migration of the classic Notepad++ codebase to a modern Rust implementation while integrating the Null Claw language‑server agent.

## 1️⃣ Initialise Workspace
```powershell
// turbo
cd d:\Projects\ModernNotepadPlusPlus
cargo new --lib app
cargo new --bin nullclaw-ls
```

## 2️⃣ Configure Cargo Workspace
Edit the root `Cargo.toml` to include the workspace members:
```toml
[workspace]
members = ["app", "nullclaw-ls"]
```

## 3️⃣ Choose UI Toolkit (manual step)
- Decide between **Iced**, **Druid**, **egui**, or **tauri**.
- Add the chosen crate to `app/Cargo.toml` under `[dependencies]`.

## 4️⃣ Core Engine Development (see PLAN_1_CORE.md)
- Implement `src/core/buffer.rs` (text buffer, undo/redo).
- Add syntax highlighting (`src/highlight/mod.rs`).
- Provide `EditorEngine` trait in `src/api.rs`.

## 5️⃣ UI Layer (see PLAN_2_UI.md)
- Scaffold main window (`src/ui/main.rs`).
- Wire UI events to the engine (`src/ui/events.rs`).
- Implement theming, tabs, drag‑and‑drop.

## 6️⃣ Null Claw Agent Integration (see PLAN_3_NULLCLAW.md)
```powershell
// turbo
cargo run --bin nullclaw-ls --release
```
- Define LSP capabilities in `nullclaw-ls/src/protocol.rs`.
- Bridge editor events to LSP (`nullclaw-ls/src/bridge.rs`).
- Add diagnostics pane in UI.

## 7️⃣ Plugin System (see PLAN_4_PLUGINS.md)
- Design plugin API (`src/plugin/api.rs`).
- Implement manager (`src/plugin/manager.rs`).
- Port a sample plugin (e.g., XML Tools).

## 8️⃣ CI / Testing Setup
Create `.github/workflows/ci.yml` with:
- `cargo fmt -- --check`
- `cargo clippy -- -D warnings`
- `cargo test --all`
- Build release binaries.

## 9️⃣ Release & Installer
- Use **cargo-wix** or **WiX** to produce an MSI.
- Tag the repository (e.g., `v0.1.0`).
- Publish release notes.

---
*All `// turbo`‑annotated command steps can be auto‑executed by the workflow runner.*
