#[cfg(windows)]
extern crate winapi;

#[cfg(unix)]
extern crate gtk;

pub mod pal;
pub mod gui;