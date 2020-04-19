#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate wchar;

#[cfg(unix)]
extern crate gtk;

pub mod pal;
pub mod gui;