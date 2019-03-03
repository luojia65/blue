mod os;
mod addr;
mod class;

pub use class::Class;
pub use addr::Addr;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct RadioInfo {
    inner: os::RadioInfo,
}

impl RadioInfo {
    pub fn addr(&self) -> Addr {
        self.inner.addr()
    }

    pub fn name(&self) -> &str {
        self.inner.name()
    }

    pub fn class(&self) -> u32 {
        self.inner.class()
    }

    pub fn subversion(&self) -> u16 {
        self.inner.subversion()
    }

    pub fn manufacturer(&self) -> u16 {
        self.inner.manufacturer()
    }
}

impl std::fmt::Debug for RadioInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RadioInfo")
            .field("addr", &self.addr())
            .field("name", &self.name())
            .field("class", &format_args!("0x{:X}", self.class()))
            .field("subversion", &format_args!("0x{:X}", self.subversion()))
            .field("manufacturer", &format_args!("0x{:X}", self.manufacturer()))
            .finish()
    }
}

#[derive(Debug)]
pub struct Radio {
    inner: os::Radio,
}

impl Radio {
    pub fn info(&self) -> RadioInfo {
        RadioInfo { inner: self.inner.info() }
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