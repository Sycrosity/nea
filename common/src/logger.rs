use core::str::FromStr;

#[cfg(feature = "esp")]
use esp_println::println;
use log::LevelFilter;

const LOG_TARGETS: Option<&'static str> = option_env!("ESP_LOGTARGETS");

const RESET: &str = "\u{001B}[0m";
const RED: &str = "\u{001B}[31m";
const GREEN: &str = "\u{001B}[32m";
const YELLOW: &str = "\u{001B}[33m";
const BLUE: &str = "\u{001B}[34m";
const CYAN: &str = "\u{001B}[35m";

fn setup_print(data: &'static str) {
    println!("[{}{}{}] {}", GREEN, "LOGGER", RESET, data);
}

pub fn init_logger(level: LevelFilter) {
    unsafe {
        log::set_logger_racy(&Logger).unwrap();
        log::set_max_level_racy(level);
    }
}

pub fn init_logger_from_env() {
    const LEVEL: Option<&'static str> = option_env!("ESP_LOGLEVEL");

    unsafe {
        log::set_logger_racy(&Logger).unwrap();
    }

    if let Some(lvl) = LEVEL {
        let level = LevelFilter::from_str(lvl).unwrap_or(LevelFilter::Off);
        unsafe { log::set_max_level_racy(level) };
    }

    setup_print("Logger is setup");
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    #[allow(unused)]
    fn log(&self, record: &log::Record) {
        if let Some(targets) = LOG_TARGETS {
            if targets
                .split(',')
                .filter_map(|line| line.find('=').map(|equals_pos| line.split_at(equals_pos)))
                .any(|(target, level)| {
                    record.target().starts_with(target)
                        && LevelFilter::from_str(&level[1..]).unwrap_or(LevelFilter::Off)
                            < record.level()
                })
            {
                return;
            };
        }

        let color = match record.level() {
            log::Level::Error => RED,
            log::Level::Warn => YELLOW,
            log::Level::Info => GREEN,
            log::Level::Debug => BLUE,
            log::Level::Trace => CYAN,
        };
        let reset = RESET;

        println!(
            "[{}{} {} {}s{}] {}",
            color,
            record.level(),
            record.target(),
            embassy_time::Instant::now().as_secs(),
            reset,
            record.args()
        );
    }

    fn flush(&self) {}
}
