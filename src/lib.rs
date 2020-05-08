extern crate cross_gui_derive;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate wchar;

#[cfg(unix)]
extern crate gtk;
#[cfg(unix)]
extern crate glib;

pub mod pal;
pub mod gui;