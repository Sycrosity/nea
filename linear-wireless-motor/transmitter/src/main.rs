#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

mod blink;

use common::prelude::*;
use embassy_executor::Spawner;
use embassy_futures::select::select;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{AnyInput, AnyOutput, Io, Level, Pull},
    peripherals::Peripherals,
    rng::Rng,
    system::SystemControl,
    timer::timg::TimerGroup,
};
use esp_wifi::{
    esp_now::{EspNowSender, BROADCAST_ADDRESS},
    initialize, EspWifiInitFor,
};
use shared::Direction;

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

    esp_hal::interrupt::enable(
        esp_hal::peripherals::Interrupt::GPIO,
        esp_hal::interrupt::Priority::Priority1,
    )
    .unwrap();

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

    // Create a shared Mutable reference to the sender, allowing multiple buttons to
    // send messages independently
    let sender = mk_static!(
        Mutex::<NoopRawMutex, EspNowSender<'static>>,
        Mutex::<NoopRawMutex, _>::new(sender)
    );

    let a = AnyInput::new(io.pins.gpio3, Pull::Up);
    let b = AnyInput::new(io.pins.gpio2, Pull::Up);

    spawner.must_spawn(blink::blink(AnyOutput::new(io.pins.gpio8, Level::Low)));

    spawner.must_spawn(broadcaster(sender, a, b));
}

/// Task that listens for button presses and sends a message when a button is
/// pressed.
#[embassy_executor::task(pool_size = 2)]
async fn broadcaster(
    sender: &'static Mutex<NoopRawMutex, EspNowSender<'static>>,
    mut pin_a: AnyInput<'static>,
    mut pin_b: AnyInput<'static>,
) {
    let mut count = 0;

    enum State {
        Locked,
        TurnRightStart,
        TurnRightMiddle,
        TurnRightEnd,
        TurnLeftStart,
        TurnLeftMiddle,
        TurnLeftEnd,
        Undecided,
    }

    let mut state = State::Locked;

    loop {
        let mut maybe_direction: Option<Direction> = None;

        select(pin_a.wait_for_any_edge(), pin_b.wait_for_any_edge()).await;
        let (is_a_low, is_b_low) = (pin_a.is_low(), pin_b.is_low());

        match state {
            State::Locked => {
                if is_a_low && is_b_low {
                    state = State::Undecided;
                } else if is_a_low && !is_b_low {
                    state = State::TurnRightStart;
                } else if !is_a_low && is_b_low {
                    state = State::TurnLeftStart;
                } else {
                    state = State::Locked;
                }
            }
            State::TurnRightStart => {
                if is_a_low && is_b_low {
                    state = State::TurnRightMiddle;
                } else if !is_a_low && is_b_low {
                    state = State::TurnRightEnd;
                } else if is_a_low && !is_b_low {
                    state = State::TurnRightStart;
                } else {
                    state = State::Locked;
                }
            }
            State::TurnRightMiddle | State::TurnRightEnd => {
                if is_a_low && is_b_low {
                    state = State::TurnRightMiddle;
                } else if !is_a_low && is_b_low {
                    state = State::TurnRightEnd;
                } else if is_a_low && !is_b_low {
                    state = State::TurnRightStart;
                } else {
                    state = State::Locked;
                    maybe_direction = Some(Direction::CounterClockwise);
                }
            }

            State::TurnLeftStart => {
                if is_a_low && is_b_low {
                    state = State::TurnLeftMiddle;
                } else if !is_a_low && is_b_low {
                    state = State::TurnLeftStart;
                } else if is_a_low && !is_b_low {
                    state = State::TurnLeftEnd;
                } else {
                    state = State::Locked;
                }
            }

            State::TurnLeftMiddle | State::TurnLeftEnd => {
                if is_a_low && is_b_low {
                    state = State::TurnLeftMiddle;
                } else if !is_a_low && is_b_low {
                    state = State::TurnLeftStart;
                } else if is_a_low && !is_b_low {
                    state = State::TurnLeftEnd;
                } else {
                    state = State::Locked;
                    maybe_direction = Some(Direction::Clockwise);
                }
            }

            State::Undecided => {
                if is_a_low && is_b_low {
                    state = State::Undecided;
                } else if !is_a_low && is_b_low {
                    state = State::TurnRightEnd;
                } else if is_a_low && !is_b_low {
                    state = State::TurnLeftEnd;
                } else {
                    state = State::Locked;
                }
            }
        }

        let Some(direction) = maybe_direction else {
            continue;
        };

        match direction {
            Direction::Clockwise => count += 1,
            Direction::CounterClockwise => count -= 1,
        }

        info!("count: {}", count);

        // Lock the sender mutex to allow the task to send a message
        let mut sender = sender.lock().await;

        // Send the button press as a single byte message
        match sender
            .send_async(&BROADCAST_ADDRESS, &[direction.into()])
            .await
        {
            Ok(()) => info!("{} rotation sent", direction),
            Err(e) => warn!("Couldn't send {} rotation: {e:?}", direction),
        }

        // Debounce the button, preventing it from accidentally counting as being
        // pressed again too quickly.
        Timer::after_millis(30).await;
    }
}
