# 🌟 `niti-hal-template` - Jumpstart Your AVR Projects 🚀  

[`cargo-generate`] template for kickstarting firmware development on popular AVR microcontroller boards, including the **Niti V1 Board**, designed for enhanced ease of use and performance!  

![Niti V1 Demo](https://niti-website-beta.vercel.app/assets/rust-logo-DgTKb-JD.gif)  

---

🎯 **Supported Boards**:  
- **🌟 Niti V1 Board (Recommended)**  
- Arduino Mega 2560  
- Arduino Nano  
- Arduino Nano (New Bootloader, post-January 2018)  
- Arduino Uno  

---

## 🌟 Why Choose Niti V1?  

The **Niti V1 Board** is designed with simplicity and efficiency in mind, offering:  
- **Seamless integration** with the `niti-hal` ecosystem.  
- **Improved performance** for AVR-based applications.  
- **Community support** and continuous updates via the [`niti-hal`](https://github.com/cyberkutti-iedc/niti-hal) project.  

---

## 🚀 Getting Started  

Follow these steps to quickly set up your development environment:  

### 1️⃣ Install Required Tools  

Ensure you have the following tools installed:  

- **[`cargo-generate`]**: A Rust project generator for templated workflows.  
- **[`waterman`]**: A tool to seamlessly flash your board using the `cargo` workflow.  

```bash
cargo install cargo-generate
cargo install waterman
```

### 2️⃣ Create Your Project  

Run the following command to generate your project from this template:  

```bash
cargo generate --git https://github.com/cyberkutti-iedc/niti-hal-template.git
```  

You’ll be prompted to select your target board. **For the best experience, select "Niti V1 Board" as your board.**  

---

## 🛠️ Build & Flash  

Once your project is created, building and flashing is straightforward:  

```bash
# Build and flash the firmware
cargo run
```  

If everything is set up correctly, you should see your program running on the board!  

![Flashing the Niti V1](https://niti-website-beta.vercel.app/assets/flashing-demo.gif)  

> 📄 For more detailed setup instructions, refer to the [Setup Guide](https://github.com/cyberkutti-iedc/niti-hal/wiki/Setting-up-environment).  

---

## 📜 License  

This project is available under the following licenses:  

- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))  
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))  

You can choose to use either license for your work.  

---

## 🤝 Contribution  

Contributions are welcome! By contributing to this project, you agree that your contributions will be dual-licensed under the Apache-2.0 and MIT licenses.  

---

## 🔗 Related Projects  

- [`niti-hal`](https://github.com/cyberkutti-iedc/niti-hal): The official hardware abstraction layer for Niti boards and AVR microcontrollers.  
- [`waterman`](https://github.com/cyberkutti-iedc/niti-hal/tree/next/waterman): A tool for seamless flashing within the `cargo` workflow.  

---  
