# S.T.A.L.K.E.R. Anomaly Launcher (Rust)

A lightweight, native replacement launcher for **S.T.A.L.K.E.R. Anomaly**, rewritten in Rust. Built to be fast, self-contained, and actually play nice with modern modding setups like Mod Organizer 2.

Also available on [ModDB](https://www.moddb.com/mods/stalker-anomaly/addons/anomaly-launcher).

---

## What It Adds & Technical Highlights

Beyond covering the standard launch options from the original C# launcher (DirectX 8–11 selection, shadow map sizing, `user.ltx` window modes, custom launch arguments, sound workaround, and `gamedata` addon checks), this version focuses on performance and Windows-level integration:

- **Automatic CPU Affinity:** Detects the running Anomaly process via `sysinfo` and applies a Win32 `SetProcessAffinityMask` (unchecking cores 0 & 1 on >2-core CPUs) to help reduce X-Ray engine micro-stutters, with a 60-second timeout failsafe.
- **Mod Organizer 2 (MO2) Compatible Cache Clearing:** Uses Win32 `SHFileOperationW` rather than standard filesystem calls so clearing `appdata\shaders_cache` works reliably even under MO2's virtual file system locks.
- **Multithreaded File Verification:** Runs MD5 checksum verification against `tools/checksums.md5` on a background thread, streaming live progress updates to the UI over an `mpsc` channel without freezing the interface.
- **Hardware AVX Detection:** Checks CPU support via `CPUID` and automatically disables the AVX toggle on unsupported processors.
- **Auto-Localization:** Automatically adapts to your system locale (**English**, **Russian**, and **Spanish**) using zero-allocation static string slices.
- **Persistent Config & Dual UI Modes:** Saves preferences to `launcherconfig.toml` (via `serde`/`toml`), including a `launcherregmode` option to switch between the custom borderless layout and a compact standard window.

---

## Tech Stack

- **Language:** Rust
- **GUI:** `eframe` / `egui`
- **Windows Integration:** `windows` crate (`Win32::System::Threading`, `Win32::UI::Shell`)
- **System & IO:** `sysinfo`, `sys-locale`, `walkdir`, `md5`, `rfd`, `serde` + `toml`

---

## Installation

1. Download the latest release from [ModDB](https://www.moddb.com/mods/stalker-anomaly/addons/anomaly-launcher) or the **GitHub Releases** page.
2. Place the executable in your **S.T.A.L.K.E.R. Anomaly** root folder (next to `bin/` and `appdata/`).
3. Run it directly, or point Mod Organizer 2 to it.

### Building from Source

Requires the Rust toolchain (`rustup` / `cargo`) on Windows:

```bash
git clone https://github.com/fridvp/Anomaly-Launcher.git
cd Anomaly-Launcher
cargo build --release
```

The compiled binary will be located at `target/release/AnomalyLauncher.exe`.

---

## Translations & Contributing

Want to add another language? Feel free to open a PR updating `src/localization.rs`, or reach out on Discord (`plyaka.egor`) if you'd like to help translate.