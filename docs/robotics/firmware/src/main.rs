#![no_std]
#![no_main]

use esp32c6_hal as hal;
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = hal::pac::Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = hal::clock::ClockControl::boot_defaults(system.clock_control).freeze();

    let io = hal::gpio::IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio5.into_output();

    loop {
        led.toggle().unwrap();
        hal::delay::Delay::new(&clocks).delay_ms(1000u32);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
