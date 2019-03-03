mod os;

#[derive(Debug)]
pub struct Radio {
    inner: os::Radio,
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