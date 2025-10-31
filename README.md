>[!NOTE]
>There is an official mirror of this repo on Codeberg: https://codeberg.org/DavidBalishyan/betterfetch

<div align="center">
  <h1>✨ betterfetch ✨</h1>
  <p><b>A modern, lightweight, and customizable system fetch tool</b></p>

  <img src="https://img.shields.io/github/license/betterfetch/betterfetch?style=flat-square" alt="License">
  <img src="https://img.shields.io/github/stars/betterfetch/betterfetch?style=flat-square" alt="Stars">
  <img src="https://img.shields.io/github/issues/betterfetch/betterfetch?style=flat-square" alt="Issues">
  <img src="https://img.shields.io/github/forks/betterfetch/betterfetch?style=flat-square" alt="Forks">

</div>


---

## 📖 About

**betterfetch** is a fast, lightweight, and modern alternative to tools like `neofetch`.  
It displays useful system information with style ✨ while staying minimal and highly customizable.

- ⚡ **Blazing fast** – written in **Rust**<!-- - 🖥️ **Cross-platform** – Linux, macOS, Windows(?) -->
- 🎨 **Customizable** – configure colors, ASCII art, and displayed fields
- 🧩 **Extensible** – easy to contribute new modules

--- 

## Documenatation

The documentation for `betterfetch` can be found in the `docs/` folder of the repo.


---
>[!WARNING]
>We don't host binaries for platforms other than x86_64.
>The install script builds the project on your local machine so It's recommended to use the script for the latest version and for compatability with platforms other than x86_64
## 📦 Installation
```bash
curl https://betterfetch.github.io/scripts/rs/install.sh | bash
```
---

## 🚀 Usage
```bash
betterfetch
```

You’ll see a sleek summary of your system information:
- OS, Kernel, Uptime
- CPU, Memory, GPU
- Disk usage
- Hostname, Shell, Packages

---

## User Configuration
for user configuarion see `docs/config.md`

---

## 🛠️ Development

### Requirements
- Rust (latest stable)
- Git

### Run locally
```bash
cargo run
```

---

## 🤝 Contributing
We love contributions! 🎉 Please check out [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## 📜 License
[MIT](LICENSE) © 2025 [betterfetch contributors](https://github.com/betterfetch/betterfetch/graphs/contributors)

---

<div align="center">
Made with ❤️ by <a href="https://github.com/DavidBalishyan">David Balishyan</a>
</div>
