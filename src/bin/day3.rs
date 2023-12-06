#![no_std]
#![no_main]

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;

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

    let mut led_pin = pins.gpio18.into_push_pull_output();
    let mut led_pin2 = pins.gpio19.into_push_pull_output();
    let mut led_pin3 = pins.gpio20.into_push_pull_output();

    let button1 = pins.gpio13.into_pull_down_input();
    let button2 = pins.gpio12.into_pull_down_input();
    let button3 = pins.gpio11.into_pull_down_input();

    loop {
        if button1.is_high().unwrap() {
            led_pin.set_high().unwrap()
        }

        if button2.is_high().unwrap() {
            led_pin2.set_high().unwrap();
        }

        if button3.is_high().unwrap() {
            led_pin3.set_high().unwrap();
        }

        led_pin.set_low().unwrap();
        led_pin2.set_low().unwrap();
        led_pin3.set_low().unwrap();
    }
}
