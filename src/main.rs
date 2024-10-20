#![no_std]
#![no_main]

use panic_halt as _;

#[niti_eal::entry]
fn main() -> ! {
    let dp = niti_eal::Peripherals::take().unwrap();
    let pins = niti_eal::pins!(dp);

       {% case board -%}
      {%- when "Adafruit Trinket" -%}
    let mut led = pins.d1.into_output();
      {%- when
        "Niti V1",
        "Arduino Mega 2560",
        "Arduino Nano",
        "Arduino Nano New Bootloader",
        "Arduino Uno",
      -%}
    let mut led = pins.d13.into_output();
      {%- when "SparkFun ProMicro" -%}
    let mut led = pins.led_rx.into_output();
    {%- endcase %}

    loop {
        led.toggle();
        niti_eal::delay_ms(1000);
    }
}
{%- comment %}
# vim: ft=rust.jinja2
{% endcomment %}
