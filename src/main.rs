use anyhow::Result;
use embedded_hal::{digital::v2::InputPin, prelude::*};
use esp_idf_hal::gpio::Pull;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let mut rtos_delay = esp_idf_hal::delay::FreeRtos;

    let peripherals = esp_idf_hal::peripherals::Peripherals::take().unwrap();
    let btn = peripherals.pins.gpio18;
    let mut btn = btn.into_input()?;
    btn.set_pull_up()?;

    println!("Ready for input");

    let mut last_state = false;
    loop {
        let state = btn.is_low()?;
        if state != last_state {
            if state {
                println!("pressed");
            } else {
                println!("released");
            }

            last_state = state;
        }
        rtos_delay.delay_ms(100u32);
    }
}
