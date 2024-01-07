#![no_std]
#![no_main]

use defmt_serial as _;
use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::adc::AdcPin;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;
use rp_pico::hal::Adc;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = hal::Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let tilt_pin = pins.gpio26.into_pull_down_input();
    let mut led_pin = pins.gpio18.into_push_pull_output();

    loop {
        
        if tilt_pin.is_high().unwrap() {
            led_pin.set_high().unwrap();
        }
        else {
            led_pin.set_low().unwrap();
        }

        delay.delay_ms(100);
    }
}
