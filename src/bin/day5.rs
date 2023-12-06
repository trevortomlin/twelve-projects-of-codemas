#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use defmt_serial as _;
use embedded_hal::adc::OneShot;
use embedded_hal::can::nb;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::adc::AdcPin;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pwm::FreeRunning;
use rp_pico::hal::pwm::InputHighRunning;
use rp_pico::hal::pwm::Pwm2;
use rp_pico::hal::pwm::Slice;
use rp_pico::hal::pwm::Slices;
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

    let pwm_slices = &mut Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm2;
    pwm.set_ph_correct();
    pwm.enable();
    pwm.output_to(pins.gpio21);

    let a3f = 208.; 
    let b3f = 233.; 
    let b3 = 247.;
    let c4 = 261.;
    let c4s = 277.; 
    let e4f = 311.; 
    let f4 = 349.;
    let a4f = 415.; 
    let b4f = 466.; 
    let b = 493.;
    let c5 = 523.;
    let c5s = 554.;
    let e5f = 622.; 
    let f5 = 698.;
    let f5s = 740.; 
    let a5f = 831.; 
    let rest = 0.0;

    let melody = [
        b4f, b4f, a4f, a4f, f5, f5, e5f, b4f, b4f, a4f, a4f, e5f, e5f, c5s, c5, b4f, c5s, c5s, c5s,
        c5s, c5s, e5f, c5, b4f, a4f, a4f, a4f, e5f, c5s, b4f, b4f, a4f, a4f, f5, f5, e5f, b4f, b4f,
        a4f, a4f, a5f, c5, c5s, c5, b4f, c5s, c5s, c5s, c5s, c5s, e5f, c5, b4f, a4f, rest, a4f,
        e5f, c5s, rest,
    ];

    let durations = [
        1, 1, 1, 1, 3, 3, 6, 1, 1, 1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 1, 3, 3, 3, 1, 2, 2, 2, 4, 8, 1,
        1, 1, 1, 3, 3, 6, 1, 1, 1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 1, 3, 3, 3, 1, 2, 2, 2, 4, 8, 4,
    ];

    for (i, note) in melody.iter().enumerate() {
        if *note == rest {
            delay.delay_ms(durations[i] * 100);
            continue;
        }
        play_note(pwm, &mut delay, *note / 2., durations[i] * 100);
    }

    loop {}
}

fn calc_note(freq: f32) -> u16 {
    (12_000_000 as f32 / freq) as u16
}

fn play_note(pwm: &mut Slice<Pwm2, FreeRunning>, delay: &mut Delay, freq: f32, duration: u32) {
    let note = calc_note(freq);
    pwm.channel_b.set_duty(250);
    pwm.set_top(note);
    delay.delay_ms(duration);
    pwm.channel_b.set_duty(0);
}
