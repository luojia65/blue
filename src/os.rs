#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::{Radio, RadioInfo, Radios, radios};

#[cfg(unix)]
pub mod unix;

#[cfg(unix)]
pub use unix::{Radio, RadioInfo, Radios, radios};
