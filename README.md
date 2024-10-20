# Niti-EAL Template  
========================================

### 🚀 Start Your Embedded Project with Niti-EAL, the Waterman Way! 🚀

Welcome to the **Niti-EAL Template** – your starting point for building amazing embedded systems on AVR-based boards using **Rust**! This template helps you jump into development with ease, thanks to the power of **Niti-EAL** and **Waterman**.

---

## Supported Boards 🎛️

This template is designed to work with the following microcontroller boards:

- 🛠️ **Niti V1** (Perfect for your projects!)
- 🛠️ **Arduino Mega 2560**
- 🛠️ **Arduino Nano**
- 🛠️ **Arduino Nano (New Bootloader)** (After January 2018)
- 🛠️ **Arduino Uno**

---

## Getting Started 🏁

Follow these easy steps to get your project up and running in no time:

### 1. Install Prerequisites 🔧

You'll need to have the following tools installed on your system:

```bash
cargo install cargo-generate
cargo install waterman
```

- **cargo-generate**: Used for generating the project template.
- **waterman**: The flashing and serial communication tool for your AVR board.

### 2. Create a New Project 📦

Use the **Niti-EAL Template** to create your project:

```bash
cargo generate --git "https://github.com/cyberkutti-iedc/Niti-eal-template.git"
```

You’ll be prompted to select your target board. Pick the one you're using (e.g., **Niti V1**) and you're ready to go!

### 3. Build and Flash ⚡

Now that your project is set up, build and flash it to your Niti board with a simple command:

```bash
cargo run
```

🔄 This will build your project, flash it to your board, and open up a UART console via **Waterman** for you to interact with your device.

---

## Why Choose Niti-EAL? 🤔

Niti-EAL (Embedded Abstraction Layer) brings powerful abstraction and simplicity to embedded development in **Rust**. Whether you're working with sensors, LEDs, or serial communication, Niti-EAL makes it easy and fun!

- 💡 **Easy to Use**: Simplified setup for quick start projects.
- ⚙️ **Built for Rust**: Rust’s powerful features combined with seamless board integration.
- 🚀 **Cross-Board Support**: Works across multiple AVR boards including Niti V1, Arduino Uno, and more.
- 🛠️ **Smooth Flashing & UART**: Flash firmware and monitor console output in one go with **Waterman**.

---

## Project Structure 📂

When you generate a project using this template, you'll find the following structure:

- **niti-eal/**: The core abstraction library that simplifies access to the hardware.
- **examples/**: A set of example projects to get you started, from blinking LEDs to advanced sensor handling.
- **avr-specs/**: AVR target definitions necessary for compiling Rust code for your selected microcontroller.

---

## License 📜

This project is dual-licensed, so you can choose between:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

You may use, modify, and distribute this project under either license.

---

## Contributing 🛠️

We’d love your help! Contributions are always welcome, and all submissions are governed by the Apache 2.0 license or MIT license, at your discretion. Together, let's build something awesome!

---

### Let’s bring embedded systems to life with **Niti-EAL** and **Rust**! 🤖✨
```