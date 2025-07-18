# ratatui-flux-template

A minimal, production-ready starter that wires [Ratatui](https://ratatui.rs) to the **Flux architecture** (unidirectional data flow) using a clean, library-style layout.

---

## Features

* **Flux Pattern** – `Action → Dispatcher → Store → View` keeps state predictable.  
* **Multi-file Layout** – idiomatic `lib/mod.rs` + binaries; easy to extend.  
* **Keyboard driven** – `↑/k` increment, `↓/j` decrement, `q/Esc` quit.  
* **cargo-generate ready** – one command to scaffold a new project.

---

## Use (cargo-generate)

Install once:

```bash
cargo install cargo-generate

```
