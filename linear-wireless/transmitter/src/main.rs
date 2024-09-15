#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use common::prelude::*;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{AnyInput, Io, Pull},
    peripherals::Peripherals,
    rng::Rng,
    system::SystemControl,
    timer::timg::TimerGroup,
};
use esp_wifi::{
    esp_now::{EspNowSender, BROADCAST_ADDRESS},
    initialize, EspWifiInitFor,
};
use linear_shared::{Button, Colour};

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
    let (_, sender, _) = esp_now.split();

    // Create a shared Mutable reference to the sender, allowing multiple buttons to send messages independently
    let sender = mk_static!(
        Mutex::<NoopRawMutex, EspNowSender<'static>>,
        Mutex::<NoopRawMutex, _>::new(sender)
    );

    // Map the button colours to their respective pins
    let buttons = [
        Button::new(AnyInput::new(io.pins.gpio6, Pull::Up), Colour::Red),
        Button::new(AnyInput::new(io.pins.gpio10, Pull::Up), Colour::Yellow),
        Button::new(AnyInput::new(io.pins.gpio5, Pull::Up), Colour::Blue),
    ];

    // Spawn a broadcaster task for each button
    for button in buttons {
        info!("{} button ready.", button.colour);
        spawner.must_spawn(broadcaster(sender, button));
    }
}

/// Task that listens for button presses and sends a message when a button is pressed.
#[embassy_executor::task(pool_size = 3)]
async fn broadcaster(
    sender: &'static Mutex<NoopRawMutex, EspNowSender<'static>>,
    mut button: Button,
) {
    loop {

        button.pin.wait_for_rising_edge().await;

        trace!("{} button pressed", button.colour);

        // Lock the sender mutex to allow the task to send a message
        let mut sender = sender.lock().await;

        // Send the button press as a single byte message
        match sender
            .send_async(&BROADCAST_ADDRESS, &[button.colour.to_u8()])
            .await
        {
            Ok(()) => info!("{} button press sent", button.colour),
            Err(e) => warn!("Couldn't send {} button press: {e:?}", button.colour),
        }

        // Debounce the button, preventing it from accidentally counting as being
        // pressed again too quickly.
        Timer::after_millis(50).await;
    }
}
