#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(cfg_match)]
#![allow(unused_variables)]

use embassy_executor::Spawner;
use esp_hal::{
    clock::ClockControl,
    gpio::{Io, Level},
    peripherals::Peripherals,
    system::SystemControl,
};
use esp_println::println;
use motor_blink::{blink::blink, prelude::*};

#[main]
async fn main(spawner: Spawner) {
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/
    // rust-lang/cargo/issues/10358
    common::logger::init_logger_from_env();

    println!("Hello world!");

    let peripherals = Peripherals::take();

    let system = SystemControl::new(peripherals.SYSTEM);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let clocks = ClockControl::max(system.clock_control).freeze();

    #[cfg(feature = "esp32")]
    {
        let timg1 = TimerGroup::new(peripherals.TIMG1, &clocks);

        esp_hal_embassy::init(&clocks, timg1.timer0);
    }

    #[cfg(not(feature = "esp32"))]
    {
        let systimer = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER)
            .split::<esp_hal::timer::systimer::Target>();

        esp_hal_embassy::init(&clocks, systimer.alarm0);
    }

    // let adc1_config = AdcConfig::new();

    // let rng = Rng::new(peripherals.RNG);
    // let mut _rng = *RNG.init(rng);

    // let pot_pin = io.pins.gpio3;
    // let pot_pin = adc1_config.enable_pin::<_>(pot_pin,
    // Attenuation::Attenuation11dB); let pot_pin =
    // adc1_config.enable_pin_with_cal::<_, AdcCalCurve<ADC1>>(pot_pin,
    // Attenuation::Attenuation11dB);

    // let _adc1 =
    //     &*SHARED_ADC.init_with(|| Mutex::new(Adc::<ADC1>::new(peripherals.ADC1,
    // adc1_config)));

    #[cfg(feature = "esp32")]
    let internal_led = AnyOutput::new(io.pins.gpio2, Level::Low);
    #[cfg(feature = "esp32c3")]
    let internal_led = AnyOutput::new(io.pins.gpio8, Level::Low);

    spawner.must_spawn(blink(internal_led));

    // let motor_pin = io.pins.gpio32;

    let motor_pin_l = io.pins.gpio9;
    let motor_pin_r = io.pins.gpio3;

    // let motor_pin = io.pins.gpio25;

    spawner.must_spawn(blink(AnyOutput::new(motor_pin_r, Level::High)));

    spawner.must_spawn(blink(AnyOutput::new(motor_pin_l, Level::Low)));
}
