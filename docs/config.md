# Betterfetch Configuration
Betterfetch supports a user configuration file to customize ASCII art, shown modules, and colors.

## 📂 Config File Location

By default, Betterfetch looks for:

```bash
$HOME/.config/betterfetch/config.toml
```
If the file doesn’t exist, defaults are used.

## ⚙️ Config Format

The config file uses [TOML](https://toml.io)
```toml
# Custom ASCII art (triple single quotes recommended to avoid escaping backslashes)
ascii_art = ''' 
 _          _   _             __      _       _
| |__   ___| |_| |_ ___ _ __ / _| ___| |_ ___| |__
| '_ \ / _ \ __| __/ _ \ '__| |_ / _ \ __/ __| '_ \
| |_) |  __/ |_| ||  __/ |  |  _|  __/ || (__| | | |
|_.__/ \___|\__|\__\___|_|  |_|  \___|\__\___|_| |_|

'''

# Modules to display, in order
modules = ["user", "host", "os", "kernel", "cpu", "memory", "disk", "packages"]

# Color settings
[colors]
title = "yellow"   # Color for field names
```

## 🎨 Colors

Colors are powered by the [colored](https://crates.io/crates/colored) crate.
<br/>
You can use basic color names:

- black
- red
- green
- yellow
- blue
- magenta
- cyan
- white


## 🔧 Modules

Available modules:

- user - Current username
- host - Hostname
- os - OS + distro line
- kernel - Kernel version
- uptime - System uptime
- cpu - CPU name + cores
- memory - Used / total memory
- disk - Disk usage line
- packages - Installed package count

## 🖼 ASCII Art

Use ''' triple single quotes for ASCII art (best for backslashes).

Or escape backslashes with \\ inside """ triple double quotes.

Example (valid):

```toml
ascii_art = ''' 
 _          _   _             __      _       _
| |__   ___| |_| |_ ___ _ __ / _| ___| |_ ___| |__
| '_ \ / _ \ __| __/ _ \ '__| |_ / _ \ __/ __| '_ \
| |_) |  __/ |_| ||  __/ |  |  _|  __/ || (__| | | |
|_.__/ \___|\__|\__\___|_|  |_|  \___|\__\___|_| |_|

'''
```

## 📌 Roadmap

Planned config features:

- Per-field colors (different color for titles and values)
- Theming support (switchable presets)
- Custom module definitions


**User configuation support has been added in v0.1.3**