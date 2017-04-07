#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate user32;

#[cfg(unix)]
extern crate gtk;

pub mod pal;
pub mod gui;