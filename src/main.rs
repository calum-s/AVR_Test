#![no_std]
#![no_main]

use arduino_hal::prelude::_void_ResultVoidExt;
use panic_halt as _;

struct RocketState {}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello\r").void_unwrap();

    loop {
        led.toggle();
        arduino_hal::delay_ms(3000);
    }
}
