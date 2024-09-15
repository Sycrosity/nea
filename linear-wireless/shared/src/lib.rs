#![cfg_attr(not(test), no_std)]
use core::fmt::Display;
#[cfg(test)]
use std::prelude::rust_2021::*;

use esp_hal::gpio::{AnyInput, AnyOutput};

const RESET: &str = "\u{001B}[0m";
const RED: &str = "\u{001B}[31m";
const YELLOW: &str = "\u{001B}[33m";
const BLUE: &str = "\u{001B}[36m";

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Colour {
    Red = 0,
    Yellow = 1,
    Blue = 2,
}

impl Display for Colour {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Red => write!(f, "{}Red{}", RED, RESET),
            Self::Yellow => write!(f, "{}Yellow{}", YELLOW, RESET),
            Self::Blue => write!(f, "{}Blue{}", BLUE, RESET),
        }
    }
}

impl Colour {
    pub const ALL: [Self; 3] = [Self::Red, Self::Blue, Self::Yellow];

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Red),
            1 => Some(Self::Yellow),
            2 => Some(Self::Blue),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

pub struct Led {
    pub pin: AnyOutput<'static>,
    pub colour: Colour,
}

impl Led {
    pub fn new(pin: AnyOutput<'static>, colour: Colour) -> Self {
        Self { pin, colour }
    }
}

pub struct Button {
    pub pin: AnyInput<'static>,
    pub colour: Colour,
}

impl Button {
    pub fn new(pin: AnyInput<'static>, colour: Colour) -> Self {
        Self { pin, colour }
    }
}
