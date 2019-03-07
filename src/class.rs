// https://www.bluetooth.com/specifications/assigned-numbers/baseband
#![allow(unused)]

use core::fmt;

const MAJOR_SERVICE_POSITIONING: u32 = 1 << 16;
const MAJOR_SERVICE_NETWORKING: u32 = 1 << 17;
const MAJOR_SERVICE_RENDERING: u32 = 1 << 18;
const MAJOR_SERVICE_CAPTURING: u32 = 1 << 19;
const MAJOR_SERVICE_OBJECT_TRANSFER: u32 = 1 << 20;
const MAJOR_SERVICE_AUDIO: u32 = 1 << 21;
const MAJOR_SERVICE_TELEPHONY: u32 = 1 << 22;
const MAJOR_SERVICE_INFORMATION: u32 = 1 << 23;

const MAJOR_DEVICE_COMPUTER: u32 = 0b00001 << 8;
const MAJOR_DEVICE_PHONE: u32 = 0b00010 << 8;
const MAJOR_DEVICE_NETWORK_ACCESS_POINT: u32 = 0b00011 << 8;
const MAJOR_DEVICE_AUDIO_VIDEO: u32 = 0b00100 << 8;
const MAJOR_DEVICE_PHRIPHERAL: u32 = 0b00101 << 8;
const MAJOR_DEVICE_IMAGING: u32 = 0b00110 << 8;
const MAJOR_DEVICE_WEARABLE: u32 = 0b00111 << 8;
const MAJOR_DEVICE_TOY: u32 = 0b01000 << 8;
const MAJOR_DEVICE_HEALTH: u32 = 0b01001 << 8;
const MAJOR_DEVICE_UNCATEGORIZED: u32 = 0b11111 << 8;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Class {
    value: u32
}

impl From<u32> for Class {
    fn from(value: u32) -> Class {
        Class { value }
    }
}

impl From<Class> for u32 {
    fn from(class: Class) -> u32 {
        class.value
    }
}

macro_rules! impl_fmt {
    ($($trait_to_impl: ident )+) => {
        $(
impl fmt::$trait_to_impl for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use fmt::$trait_to_impl;
        self.value.fmt(f)
    }
} 
        )+
    };
}

impl_fmt!(Binary Debug Display LowerHex Octal UpperHex);
