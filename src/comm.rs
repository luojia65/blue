// use super::*;

// pub struct Stream {
//     // todo
// }

// impl Stream {
//     pub fn listen<A: ToBlueAddrs>(addr: A) -> Result<Stream> {
//         unimplemented!()
//     }
// }

// impl Drop for Stream {
//     fn drop(&mut self) {
//         unimplemented!()
//     }
// }

// #[cfg(not(no_std))]
// mod std_impls {
//     use super::*;
//     use std::io::{Read, Write, Result};

//     impl Read for Stream {
//         fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
//             unimplemented!()
//         }
//     }

//     impl Write for Stream {
//         fn write(&mut self, buf: &[u8]) -> Result<usize> {
//             unimplemented!()
//         }

//         fn flush(&mut self) -> Result<()> {
//             unimplemented!()
//         }
//     }
// }
