use gtk;
use gtk::{ContainerExt, WidgetExt, FixedExt, LabelExt, Align, Orientation};
use glib::object::Cast as _;
use crate::gui;
use gui::Widget;

#[derive(Clone)]
pub struct Label {
    //has a clutch for allignment without V3_16
    box_: gtk::Box,
    inner: gtk::Label
}
impl Label {
    pub fn new() -> Label {
        let box_ = gtk::Box::new(Orientation::Horizontal, 0);
        let inner = gtk::Label::new(None);
        box_.add(&inner);
        inner.set_halign(Align::Start);
        inner.set_valign(Align::Center);
        box_.show_all();
        Label{
            box_,
            inner 
        }
    }
}

impl Widget for Label {
    fn upcast(&self) -> Box<dyn Widget> {
        Box::new(self.clone())
    }
    fn get_native(&self) -> Box<dyn std::any::Any> {
        Box::new(self.box_.clone().upcast::<gtk::Widget>())
    }
    fn set_hidden(&mut self, hidden: bool) {
        if hidden {
            self.box_.show_all();
        } else {
            self.box_.hide();
        }
    }
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize) {
        self.box_.set_size_request(w as i32, h as i32);
        let parent = self.box_.get_parent();
        if let Some(parent) = parent {
            let parent = parent.dynamic_cast::<gtk::Fixed>();
            if let Ok(fixed_layout) = parent {
                fixed_layout.move_(&self.box_, x as i32, y as i32);
            }
        }
    }
    
}

#[cross_gui_derive::auto_clone]
impl gui::Label for Label {
    fn set_title(&mut self, title: &str) {
        self.inner.set_text(title);
    }
    
}