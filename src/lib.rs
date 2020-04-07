#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate user32;

#[cfg(unix)]
extern crate gtk;

#[macro_use]
extern crate lazy_static;

pub mod pal;
pub mod gui;