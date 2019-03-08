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
    pub fn info(&self) -> RadioInfo {
        self.inner.info()
    }
}

impl fmt::Debug for Radio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.info())
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