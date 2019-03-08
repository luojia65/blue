mod os;
mod addr;
mod class;
mod manufacturer;

use core::fmt;

pub use class::Class;
pub use addr::Addr;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct RadioInfo {
    addr: Addr,
    name: String,
    class: Class,
    subversion: u16,
    manufacturer: u16,
}

impl RadioInfo {
    pub fn new() -> RadioInfo {
        RadioInfo {
            addr: Addr::from([0, 0, 0, 0, 0, 0]),
            name: String::from(""),
            class: Class::from(0),
            subversion: 0,
            manufacturer: 0,
        }
    }

    pub fn addr(&self) -> Addr {
        self.addr
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn class(&self) -> Class {
        self.class
    }

    pub fn subversion(&self) -> u16 {
        self.subversion
    }

    pub fn manufacturer(&self) -> u16 {
        self.manufacturer
    }
}

pub struct Radio {
    inner: os::Radio,
}

impl Radio {
    pub fn read_info(&self, buf: &mut RadioInfo) {
        self.inner.read_info(buf)
    }
}

impl fmt::Debug for Radio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = RadioInfo::new();
        self.read_info(&mut buf);
        write!(f, "{:?}", buf)
    }
}

#[derive(Debug)]
pub struct Radios {
    inner: os::Radios
}

impl Iterator for Radios {
    type Item = Radio;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|inner| Radio { inner })
    }
}

pub fn radios() -> Radios {
    Radios {
        inner: os::radios()
    }
}