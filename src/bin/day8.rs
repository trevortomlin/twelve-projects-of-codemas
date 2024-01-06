/*
IMPORTANT: Needs to be run in release mode otherwise you will get OneWireError::UnexpectedResponse
*/

#![no_std]
#![no_main]

use ds18b20::{Ds18b20, Resolution, SensorData};
use embedded_hal::{digital::v2::{OutputPin, InputPin}, blocking::delay::{DelayMs, DelayUs}};
use heapless::String;
use rp_pico::{entry, hal::{Clock, pio::PinState}, Pins};
use panic_halt as _;
use rp_pico::hal::pac;
use rp_pico::hal;
use usb_device::{class_prelude::*, prelude::*, device};
use usbd_serial::SerialPort;
use one_wire_bus::{OneWire, OneWireError, OneWireResult};
use core::fmt::Debug;

#[derive(Debug)]
struct Error;

#[entry]
fn main() -> ! {
    let core = pac::CorePeripherals::take().unwrap();
    let mut pac = pac::Peripherals::take().unwrap();

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

    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Trevor")
        .product("Serial port")
        .serial_number("0")
        .device_class(2)
        .build();

    let mut onboard_led_pin = pins.led.into_push_pull_output();
    let mut led_red = pins.gpio18.into_push_pull_output();
    let mut led_yellow = pins.gpio19.into_push_pull_output();
    let mut led_green = pins.gpio20.into_push_pull_output();


    let mut one_wire_pin = hal::gpio::InOutPin::new(pins.gpio26);

    let mut one_wire_bus = OneWire::new(one_wire_pin).unwrap();

    let mut last_temp: f32 = 0.;

    let mut num_to_led = |num: u8| {
        if num >= 6 {
            return;
        }
        led_red.set_low();
        led_yellow.set_low();
        led_green.set_low();
        if num & 1  == 1 {
            led_red.set_high();
        }
        if (num >> 1) & 1  == 1 {
            led_yellow.set_high();
        }
        if (num >> 2) & 1  == 1 {
            led_green.set_high();
        }
    };

    loop {
        let _ = match get_temperature(&mut delay, &mut one_wire_bus) {
            Ok(sensor_data) => {
                if onboard_led_pin.is_high().unwrap() {
                    onboard_led_pin.set_low();
                }  

                let temp = sensor_data.temperature;

                if temp - last_temp > 1.0 {
                    led_green.set_high();
                }
                else if temp - last_temp < -1.0 {
                    led_red.set_high();
                }
                else {
                    led_yellow.set_high();
                }

                last_temp = temp;


            },
            Err(_) => {
                onboard_led_pin.set_high();
            }
        };
        delay.delay_ms(5000);
        led_green.set_low();
        led_yellow.set_low();
        led_red.set_low();
    }

}

fn get_temperature<P, E>(
    delay: &mut (impl DelayUs<u16> + DelayMs<u16>),
    one_wire_bus: &mut OneWire<P>,
) -> OneWireResult<SensorData, E>
    where
        P: OutputPin<Error=E> + InputPin<Error=E>,
        E: Debug
{

    ds18b20::start_simultaneous_temp_measurement(one_wire_bus, delay)?;

    Resolution::Bits12.delay_for_measurement_time(delay);

    let mut search_state = None;
    loop {
        if let Some((device_address, state)) = one_wire_bus.device_search(search_state.as_ref(), false, delay)? {
            search_state = Some(state);
            if device_address.family_code() != ds18b20::FAMILY_CODE {
                continue;
            }
            let sensor = Ds18b20::new(device_address)?;

            let sensor_data = sensor.read_data(one_wire_bus, delay)?;

            return Ok(sensor_data) 
        } else {
            break;
        }
    }
    return Err(OneWireError::Timeout);
}