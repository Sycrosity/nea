#![cfg_attr(not(test), no_std)]
use core::fmt::Display;
#[cfg(test)]
use std::prelude::rust_2021::*;

use embedded_hal::digital::{InputPin, OutputPin};

const RESET: &str = "\u{001B}[0m";
const RED: &str = "\u{001B}[31m";
// const YELLOW: &str = "\u{001B}[33m";
const BLUE: &str = "\u{001B}[36m";

// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct Angle(pub u8);

// impl Display for Angle {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         ((self.0 as f32) * TAU) / 256f32
//     }
// }

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left = 0,
    Right = 1,
}

impl Display for Direction {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Left => write!(f, "{}Left{}", RED, RESET),
            Self::Right => write!(f, "{}Right{}", BLUE, RESET),
        }
    }
}

impl Direction {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Left),
            1 => Some(Self::Right),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[test]
fn test_input() {
    assert_eq!(Direction::from_u8(0), Some(Direction::Left));
    assert_eq!(Direction::from_u8(1), Some(Direction::Right));
    assert_eq!(Direction::from_u8(2), None);
    assert_eq!(Direction::Left.to_u8(), 0);
    assert_eq!(Direction::Right.to_u8(), 1);
}

pub struct Motor<Pin: OutputPin> {
    pub pin: Pin,
    pub direction: Direction,
}

impl<Pin: OutputPin> Motor<Pin> {
    pub fn new(pin: Pin, direction: Direction) -> Self {
        Self { pin, direction }
    }
}

pub struct Button<Pin: InputPin> {
    pub pin: Pin,
    pub direction: Direction,
}

impl<Pin: InputPin> Button<Pin> {
    pub fn new(pin: Pin, direction: Direction) -> Self {
        Self { pin, direction }
    }
}
