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
use rp_pico::hal::Spi;
use rp_pico::hal::adc::AdcPin;
use rp_pico::hal::gpio::FunctionI2C;
use rp_pico::hal::gpio::FunctionPio0;
use rp_pico::hal::gpio::Pin;
use rp_pico::hal::gpio::PullDown;
use rp_pico::hal::gpio::bank0::Gpio19;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;
use rp_pico::hal::Adc;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x64;
use core::fmt::Write;
use ssd1306::{mode::TerminalMode, prelude::*, I2CDisplayInterface, Ssd1306};
use hal::fugit::RateExtU32;
use pio_proc::pio_asm;

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

    let led_pin = pins.gpio18.into_push_pull_output();
    let neopixel_pin: Pin<_, FunctionPio0, _> = pins.gpio28.into_function();
    let neopixel_pin_id = neopixel_pin.id().num;

    let program = pio_proc::pio_asm!(
        ".side_set 1",
        ".wrap_target",
        "bitloop:",
            "out x 1 side 0 [2]",
            "jmp !x do_zero side 1 [1]",
        "do_one:",
            "jmp bitloop side 1 [4]",
        "do_zero:",
            "nop side 0 [4]",
        ".wrap",
        options(max_program_size = 32)
    );

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let installed = pio.install(&program.program).unwrap();
    let (mut sm, _, mut tx) = hal::pio::PIOBuilder::from_program(installed)
        .out_shift_direction(hal::pio::ShiftDirection::Left)
        .pull_threshold(32)
        .autopull(true)
        .side_set_pin_base(neopixel_pin_id)
        .clock_divisor_fixed_point(15, 160)
        .build(sm0);
    sm.set_pindirs([(neopixel_pin_id, hal::pio::PinDir::Output)]);
    sm.start();

    // grb
    let colors = &[
        0xFF, 0xFF, 0xFF, // white
        0x00, 0xFF, 0xFF, // purple
        0xFF, 0x00, 0x00, // green
        0x00, 0x00, 0xFF, // blue
        0xFF, 0xFF, 0x00, // yellow
        0xFF, 0x00, 0xFF, // cyan
        0xFF, 0xFF, 0xFF, // white
        0x00, 0xFF, 0x00, // red
        0xa5, 0xFF, 0x00, // orange
        0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF,
        0x00, 0x00, 0x00, // Placeholders so that length % 4 == 0
    ];

    let red = &[
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0xFF, 0x00, 
        0x00, 0x00, 0x00, // Placeholders so that length % 4 == 0
    ];

    let blue = &[
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0xFF,
        0x00, 0x00, 0x00// Placeholders so that length % 4 == 0
    ];

    let pink = &[
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x14, 0xFF, 0x93,
        0x00, 0x00, 0x00// Placeholders so that length % 4 == 0
    ];

    loop {

        // -------------- Show colors in array --------------
        // for val in colors.chunks_exact(4).map(|v| (v[0] << 24) + (v[1] << 16) + (v[2] << 8) + v[3]) {
        //     tx.write(val);
        //     delay.delay_us(50);
        // }

        // -------------- Alternate red / blue --------------
        // for val in red.chunks_exact(4).map(|v| (v[0] << 24) + (v[1] << 16) + (v[2] << 8) + v[3]) {
        //     tx.write(val);
        //     delay.delay_us(50);
        // }

        // delay.delay_ms(500);

        // for val in blue.chunks_exact(4).map(|v| (v[0] << 24) + (v[1] << 16) + (v[2] << 8) + v[3]) {
        //     tx.write(val);
        //     delay.delay_us(50);
        // }

        //delay.delay_ms(500);

        // -------------- Pink --------------
        for val in pink.chunks_exact(4).map(|v| (v[0] << 24) + (v[1] << 16) + (v[2] << 8) + v[3]) {
            tx.write(val);
            delay.delay_us(50);
        }
        delay.delay_ms(10);

    }
}
