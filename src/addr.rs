use std::str::FromStr;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Addr {
    bytes: [u8; 6]
}

impl Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Addr {
        Addr::from([a, b, c, d, e, f])
    }

    pub fn bytes(self) -> [u8; 6] {
        self.bytes
    }
}

impl From<[u8; 6]> for Addr {
    fn from(bytes: [u8; 6]) -> Addr {
        Self { bytes }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ParseError {
    InvalidDigit,
    InvalidLength,
}

impl FromStr for Addr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ans = [0u8; 6];
        let mut i = 0;
        for byte_str in s.split(':') {
            if i == 6 {
                return Err(ParseError::InvalidLength)
            }
            ans[i] = u8::from_str_radix(byte_str, 16)
                .map_err(|_| ParseError::InvalidDigit)?;
            i += 1;
        }
        if i != 6 {
            return Err(ParseError::InvalidLength)
        }
        Ok(Addr::from(ans))
    }
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:<02X}:{:<02X}:{:<02X}:{:<02X}:{:<02X}:{:<02X}", 
            self.bytes[0], self.bytes[1], self.bytes[2],
            self.bytes[3], self.bytes[4], self.bytes[5]
        )
    }
}

impl std::fmt::Debug for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(&self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn addr_parse() {
        assert_eq!("11:22:33:44:55:66".parse(), Ok(Addr::new(0x11, 0x22, 0x33, 0x44, 0x55, 0x66)));
        assert!("11:22:33:44:55".parse::<Addr>().is_err());
        assert!("11:22:33:44:55:66:77".parse::<Addr>().is_err());
    }
}
