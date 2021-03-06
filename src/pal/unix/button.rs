use gtk;
use gtk::{ButtonExt, FixedExt, WidgetExt};
use crate::gui;
use glib::object::Cast;
use std::{cell::RefCell, any::Any};

#[derive(Clone)]
pub struct Button {
    inner: gtk::Button
}

impl Button {
    pub fn new() -> Button {
        let inner = gtk::Button::new();
        inner.show();
        Button{ inner }
    }
}
#[cross_gui_derive::auto_clone]
impl gui::Button for Button {
    fn set_title(&mut self, title: &str) {
        self.inner.set_label(title);
    }
    fn set_on_clicked(&mut self, on_pressed: Box<dyn FnMut()>) {
        let rcell = RefCell::new(on_pressed);
        self.inner.connect_clicked(move |_| {
            let borrow = &mut *rcell.borrow_mut();
            borrow();
        });
    }
}

impl gui::Widget for Button {
    fn upcast(&self) -> Box<dyn gui::Widget> {
        Box::new(self.clone())
    }
    fn get_native(&self) -> Box<dyn Any> {
        Box::new(self.inner.clone().upcast::<gtk::Widget>())
    }
    fn set_hidden(&mut self, hidden: bool) {
        if hidden {
            self.inner.show();
        } else {
            self.inner.hide();
        }
    }
    fn resize(&mut self, width: isize, height: isize) {
        self.inner.set_size_request(width as i32, height as i32);
    }
}
