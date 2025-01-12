#![no_std]
#![no_main]

use panic_halt as _;

#[niti_hal::entry]
fn main() -> ! {
    let dp = niti_hal::Peripherals::take().unwrap();
    let pins = niti_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/cyberkutti-iedc/niti-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    {% case board -%}
      {%- when "Adafruit Trinket" -%}
    let mut led = pins.d1.into_output();
      {%- when

        "Arduino Mega 2560",
        "Arduino Mega 1280",
        "Arduino Nano",
        "Arduino Nano New Bootloader",
        "Arduino Uno",
        "Nano168",
        
      -%}
    let mut led = pins.d13.into_output();
      {%- when "SparkFun ProMicro" -%}
    let mut led = pins.led_rx.into_output();
    {%- endcase %}

    loop {
        led.toggle();
        niti_hal::delay_ms(1000);
    }
}
{%- comment %}
# vim: ft=rust.jinja2
{% endcomment %}
