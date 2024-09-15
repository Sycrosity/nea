#![cfg_attr(not(test), no_std)]
#[cfg(test)]
use std::prelude::rust_2021::*;

pub mod errors;
pub mod logger;

/// A simplified version of [`make_static`](`static_cell::make_static`), while [rust-analyzer#13824](https://github.com/rust-lang/rust-analyzer/issues/13824) exists (due to TAIT not being stable yet: [rust#120700](https://github.com/rust-lang/rust/pull/120700)).
#[macro_export]
macro_rules! mk_static {
    ($t:ty, $val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub mod prelude {

    pub use embassy_time::{Duration, Instant, Timer};
    pub use esp_println::{print, println};
    pub use log::{debug, error, info, log, trace, warn};

    pub use crate::errors::*;
    pub use crate::mk_static;
}
