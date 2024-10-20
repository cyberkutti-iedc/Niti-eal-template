# {{ project-name }}  
{{ '=================================================' | slice: 0 , project-name.size }}

### A Rust project for the _{{ board }}_ using **Niti-EAL** and powered by **Waterman**.

---

## What is Niti?

**Niti** is an advanced hardware abstraction layer designed for **AVR microcontrollers** like the **Atmega MCU family**. Developed to simplify embedded development in **Rust**, Niti provides seamless integration with a range of AVR boards, allowing developers to write efficient, high-level code while abstracting away hardware-specific details. Whether you're working on robotics, IoT, or general embedded systems, Niti offers the tools and libraries you need for reliable firmware development.

---

## Features

- **Unified API**: Simplified access to AVR peripherals.
- **Cross-platform Support**: Works on Niti V1 and similar boards.
- **Seamless Integration with Rust**: Build and flash firmware using Cargo.
- **UART Console Support**: Easily communicate with your board via a serial console.
- **Waterman Integration**: Flash firmware and manage console output effortlessly.

---

## Build Instructions

1. **Install prerequisites** for AVR development:
   
   - **Ubuntu**:
     ```bash
     sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential
     ```

   - **MacOS**:
     ```bash
     xcode-select --install
     brew tap osx-cross/avr
     brew install avr-gcc avrdude
     ```

   - **Windows**:
     Use `winget`:
     ```bash
     winget install AVRDudes.AVRDUDE ZakKemble.avr-gcc
     ```

2. **Install Waterman** for flashing and serial communication:
   ```bash
   cargo install waterman
   ```

3. **Build the firmware** using Cargo:
   ```bash
   cargo build
   ```

4. **Flash the firmware** to the Niti V1 board:
   ```bash
   cargo run
   ```

5. After flashing, **Waterman** will open a serial console session where you can interact with the UART of your board.

---

## Project Structure

- **niti-eal**: The core abstraction layer for AVR development.
- **examples/**: A collection of ready-to-use examples for your Niti board.
- **avr-specs/**: Target definitions for AVR microcontrollers, allowing easy compilation for your target hardware.

For more details, visit the [Niti-EAL Documentation](https://github.com/cyberkutti-iedc/Niti-core).

---

## Author

This project is maintained by **Sreeraj V Rajesh**. You can reach out for any queries or contributions via:

- Email: [cyberkutti@gmail.com](mailto:cyberkutti@gmail.com)
- GitHub: [cyberkutti-iedc](https://github.com/cyberkutti-iedc)

---

## License

Licensed under either of:

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

You may choose which license to apply.

---

## Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work, as defined in the Apache-2.0 license, shall be dual licensed as above, without additional terms or conditions.

---

#### Get started with Niti, bring your embedded projects to life with Rust!
