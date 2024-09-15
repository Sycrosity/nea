#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use common::prelude::*;
use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{AnyOutput, Io, Level},
    peripherals::Peripherals,
    rng::Rng,
    system::SystemControl,
    timer::timg::TimerGroup,
};
use esp_wifi::{
    esp_now::{EspNowManager, EspNowReceiver, PeerInfo, BROADCAST_ADDRESS},
    initialize, EspWifiInitFor,
};
use linear_shared::{Colour, Led};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/
    // rust-lang/cargo/issues/10358
    common::logger::init_logger_from_env();

    let peripherals = Peripherals::take();

    let system = SystemControl::new(peripherals.SYSTEM);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let clocks = ClockControl::max(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);

    cfg_if::cfg_if! {
        if #[cfg(feature = "esp32")] {
            let timg1 = TimerGroup::new(peripherals.TIMG1, &clocks);
            esp_hal_embassy::init(&clocks, timg1.timer0);
        } else {
            use esp_hal::timer::systimer::{SystemTimer, Target};
            let systimer = SystemTimer::new(peripherals.SYSTIMER).split::<Target>();
            esp_hal_embassy::init(&clocks, systimer.alarm0);
        }
    }

    // Initialize ESP-WIFI
    let init = initialize(
        EspWifiInitFor::Wifi,
        timg0.timer0,
        Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
        &clocks,
    )
    .unwrap();

    // Define the wifi peripheral
    let wifi = peripherals.WIFI;

    // Initialize ESP-NOW
    let esp_now = esp_wifi::esp_now::EspNow::new(&init, wifi).unwrap();
    info!("esp-now version: {}", esp_now.get_version().unwrap());

    // Split the ESP-NOW peripheral into its components
    let (manager, _, receiver) = esp_now.split();

    let manager = mk_static!(EspNowManager<'static>, manager);

    // Map the Led colours to the GPIO pins
    let leds = [
        Led::new(AnyOutput::new(io.pins.gpio4, Level::Low), Colour::Red),
        Led::new(AnyOutput::new(io.pins.gpio3, Level::Low), Colour::Yellow),
        Led::new(AnyOutput::new(io.pins.gpio2, Level::Low), Colour::Blue),
    ];

    // Spawn the listener task
    spawner.must_spawn(listener(manager, receiver, leds));
}

/// A listener task that listens for incoming messages from the transmitter and toggles the corresponding LED
#[embassy_executor::task]
async fn listener(
    manager: &'static EspNowManager<'static>,
    mut receiver: EspNowReceiver<'static>,
    mut leds: [Led; 3],
) {
    loop {

        // Wait for a message to be received
        let r = receiver.receive_async().await;

        // If the message is able to be converted to a colour, toggle the corresponding LED colour
        if let Some(colour) = Colour::from_u8(r.data[0]) {
            for led in &mut leds {
                if colour == led.colour {
                    info!("{} LED toggled", led.colour);
                    led.pin.toggle();
                }
            }
        }

        // If the message is a broadcast message and the peer does not exist, add the peer for future communications
        if r.info.dst_address == BROADCAST_ADDRESS && !manager.peer_exists(&r.info.src_address) {
            manager
                .add_peer(PeerInfo {
                    peer_address: r.info.src_address,
                    lmk: None,
                    channel: None,
                    encrypt: false,
                })
                .unwrap();
            debug!("Added peer {:x?}", r.info.src_address);
        }
    }
}
