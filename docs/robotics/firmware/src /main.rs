![no_std]
![no_main]

use esp32c6_hal as hal;

#[entry]
fn main() -> ! {
    let peripherals = hal::pac::Peripherals::take();
    let mut led = hal::gpio::Pin::new(peripherals.pins.gpio5).into_output();

    loop {
        led.toggle();
        hal::delay::FreeRtos::delay_ms(1000); // Parpadeo ROS2-compatible
    }
}
