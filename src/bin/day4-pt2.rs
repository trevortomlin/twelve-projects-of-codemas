#![no_std]
#![no_main]

use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::InputPin;
use rp_pico::entry;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rp_pico::hal::Adc;
use rp_pico::hal::adc::AdcPin;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;
use defmt_serial as _;
use rp_pico::hal::pwm::InputHighRunning;
use rp_pico::hal::pwm::Slices;
use embedded_hal::PwmPin;

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

    let pwm_slices =  &mut Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm1;
    pwm.set_ph_correct();
    pwm.enable();

    let channel = &mut pwm.channel_a;
    let _ = channel.output_to(pins.gpio18);

    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);

    let mut adc_pin = AdcPin::new(pins.gpio27.into_floating_input());
    
    loop {
        let val: u16 = adc.read(&mut adc_pin).unwrap_or(0);

        channel.set_duty(val * 5);

        delay.delay_ms(100);

    }
}