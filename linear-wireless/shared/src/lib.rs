#![cfg_attr(not(test), no_std)]
use core::fmt::Display;
#[cfg(test)]
use std::prelude::rust_2021::*;

use embedded_hal::digital::{InputPin, OutputPin};

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

#[test]
fn test_colours() {
    assert_eq!(Colour::Red.to_u8(), 0);
    assert_eq!(Colour::Yellow.to_u8(), 1);
    assert_eq!(Colour::Blue.to_u8(), 2);
    assert_eq!(Colour::from_u8(0), Some(Colour::Red));
    assert_eq!(Colour::from_u8(1), Some(Colour::Yellow));
    assert_eq!(Colour::from_u8(2), Some(Colour::Blue));
    assert_eq!(Colour::from_u8(3), None);
}

pub struct Led<Pin: OutputPin> {
    pub pin: Pin,
    pub colour: Colour,
}

impl<Pin: OutputPin> Led<Pin> {
    pub fn new(pin: Pin, colour: Colour) -> Self {
        Self { pin, colour }
    }
}

pub struct Button<Pin: InputPin> {
    pub pin: Pin,
    pub colour: Colour,
}

impl<Pin: InputPin> Button<Pin> {
    pub fn new(pin: Pin, colour: Colour) -> Self {
        Self { pin, colour }
    }
}
