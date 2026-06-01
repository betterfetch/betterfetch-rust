> Date format: YYYY/MM/DD
# Release notes
## v0.1.0
The first official release of betterfetch(rust)
[https://github.com/betterfetch/betterfetch/releases/tag/releases](https://github.com/betterfetch/betterfetch/releases/tag/releases)
## v0.1.2
Added: Some minor features and bug fixes
### Features
- This is a minor update, so there is no new feature

### Bug Fixes
- Made the code generally better

### Maintenance
- Updated some dependencies

### DX
- Started using editorconfig and just
## v0.1.3 (2025-09-27)
Added: config file support
### Features
- Config file support using the toml language

### Maintenance
- Updated some dependencies

### DX
- There are not DX improvement
## v0.1.4 (2026-02-08)
### Features
- side-by-side ASCII and information
- Logos for some operating systems instead of just the name with ASCII characters

### Maintenance
- Overall maintenance

### DX
- Made the build script better for nice DX
- Utilized github actions for the codeberg mirror to automatically push the latest changes 
- Created a new way of managing to-dos

<img width="598" height="307" alt="Preview" src="https://github.com/user-attachments/assets/05da9c02-0627-4077-b83e-c22e9983fca2" />
<img width="1034" height="360" alt="Screenshot From 2026-02-08 20-59-49" src="https://github.com/user-attachments/assets/89a797b4-ee59-4ed5-826b-82051e8af03a" />

## v0.1.5 (2026-06-01)
### Bug Fixes
- CPU core count now reports logical cores instead of physical cores, fixing wrong count on hybrid CPUs (e.g. i5-1235U)
- CPU name falls back to parsing `/proc/cpuinfo` when `sysinfo` returns an empty string
- Kernel line shortened to just the version string to prevent line wrapping

### Maintenance
- Collapsed nested `if` blocks in `packages.rs` to satisfy clippy
- Removed spurious `#[allow(dead_code)]` attribute on `cpu_cores` field
