#![no_std]
#![no_main]

use defmt_serial as _;
use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::Mode;
use panic_halt as _;
use rp_pico::Gp0I2C0Sda;
use rp_pico::Gp1I2C0Scl;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::I2C;
use rp_pico::hal::adc::AdcPin;
use rp_pico::hal::gpio::FunctionI2C;
use rp_pico::hal::gpio::Pin;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;
use rp_pico::hal::Adc;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use core::fmt::Write;
use ssd1306::{mode::TerminalMode, prelude::*, I2CDisplayInterface, Ssd1306};
use hal::fugit::RateExtU32;

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

    let sda_pin: Gp0I2C0Sda = pins.gpio0.reconfigure();
    let scl_pin: Gp1I2C0Scl = pins.gpio1.reconfigure();

    let i2c = I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz(),
    );

    let mut led_pin = pins.gpio18.into_push_pull_output();

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x32,
        DisplayRotation::Rotate0,
    ).into_terminal_mode();
    display.init().unwrap();
    display.clear().unwrap();

    write!(display, "Writing text to the screen!");

    loop {
    
    }
}
