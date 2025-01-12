{{ project-name }}
{{ '=================================================' | slice: 0 , project-name.size }}

Rust project for the _{{ board }}_.

## Build Instructions
1. Install prerequisites as described in the [`niti-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`waterman`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `waterman`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/waterman>.

4. `waterman` will open a console session after flashing where you can interact
   with the UART console of your board.

[`niti-hal` README]: https://github.com/cyberkutti-iedc/niti-hal#readme
[`waterman`]: https://crates.io/crates/waterman

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
