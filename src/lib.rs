#![allow(unused)]

pub mod comm;

pub struct Error {}

pub type Result<T> = core::result::Result<T, Error>;

pub struct Device {
    mac_addr: Addr
}

pub fn scan_devices(buf: &mut [Device]) -> Result<usize> {
    unimplemented!()
}

pub struct Addr {
    inner: [u8; 6]
}

pub trait ToBlueAddrs {
    // todo
}

