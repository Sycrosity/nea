#![cfg_attr(not(test), no_std)]
#[cfg(test)]
use std::prelude::rust_2021::*;

pub mod errors;
pub mod logger;

pub mod prelude {

    pub use embassy_time::{Duration, Instant, Timer};
    pub use esp_println::{print, println};
    pub use log::{debug, error, info, log, trace, warn};

    pub use crate::errors::*;
}
