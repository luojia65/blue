#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub(crate) use windows::*;

#[cfg(unix)]
pub mod unix;

#[cfg(unix)]
pub(crate) use windows::*;
