use esp_hal::gpio::AnyOutput;

use crate::prelude::*;

#[task(pool_size = 2)]
pub async fn blink(mut led: AnyOutput<'static>) {
    loop {
        led.toggle();

        if led.is_set_high() {
            info!("ON!");
        } else {
            info!("OFF!");
        }

        Timer::after(Duration::from_millis(1000)).await;
    }
}
