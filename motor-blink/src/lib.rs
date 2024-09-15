#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(cfg_match)]
#![allow(clippy::unused_unit, clippy::const_is_empty)]

pub mod blink;

pub mod prelude {

    pub use core::f64::consts::PI;

    pub use common::errors::*;
    pub use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
    pub use embassy_sync::{
        blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex},
        mutex::Mutex,
        pubsub::PubSubChannel,
        signal::Signal,
    };

    pub type SharedI2C =
        I2cDevice<'static, NoopRawMutex, I2C<'static, esp_hal::peripherals::I2C0, Async>>;

    pub static I2C_BUS: StaticCell<I2cBusMutex> = StaticCell::new();

    pub type I2cBusMutex = Mutex<NoopRawMutex, I2C<'static, esp_hal::peripherals::I2C0, Async>>;

    pub static SHARED_ADC: StaticCell<ADCMutex> = StaticCell::new();

    pub type ADCMutex = Mutex<CriticalSectionRawMutex, Adc<'static, esp_hal::peripherals::ADC1>>;

    pub static RNG: StaticCell<Rng> = StaticCell::new();

    pub use embassy_executor::task;
    pub use embassy_time::{Delay, Duration, Instant, Ticker, Timer};
    #[allow(unused)]
    pub use esp_backtrace as _;
    pub use esp_hal::{
        analog::adc::Adc,
        gpio::{Analog, AnyInput, AnyOutput, Input, Output},
        i2c::I2C,
        prelude::*,
        rng::Rng,
        Async,
    };
    pub use esp_println::{print, println};
    // pub use heapless::String;
    pub use log::{debug, error, info, log, trace, warn};
    pub use nb::block;
    pub use static_cell::{make_static, StaticCell};
}
