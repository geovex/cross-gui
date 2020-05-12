use gtk::{GtkWindowExt, WidgetExt, ContainerExt, FixedExt};
use std::{cell::RefCell, rc::Rc, any::Any};
use crate::gui;
#[derive(Clone)]
pub struct Window {
    window: gtk::Window,
    fixed: gtk::Fixed,
    user_data: Rc<RefCell<UserData>>,
}

struct UserData {
    exit_on_close: bool,
}

impl UserData {
    pub fn new() -> UserData {
        UserData {
            exit_on_close: true,
        }
    }
}

impl Window {
    pub fn new() -> Window {
        let gtk_window = gtk::Window::new(gtk::WindowType::Toplevel);
        let user_data = Rc::new(RefCell::new(UserData::new()));
        let user_data_clone = user_data.clone();
        gtk_window.connect_destroy(move |_| {
            if user_data_clone.borrow().exit_on_close {
                gtk::main_quit()
            }
        });
        let fixed_layout = gtk::Fixed::new();
        gtk_window.add(&fixed_layout);
        gtk_window.show_all();
        Window {
            user_data,
            window: gtk_window,
            fixed: fixed_layout
        }
    }
}

impl gui::Window for Window {
    fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }
    fn set_exit_on_close(&mut self, exit: bool) {
        self.user_data.borrow_mut().exit_on_close = exit;
    }
    fn add_widget(&mut self, widget: Box<dyn gui::Widget>) {
        let native_any = widget.get_native();
        //dbg!(native_any.downcast::<gtk::Widget>());
        let native: &gtk::Widget = native_any.as_ref().downcast_ref().unwrap();
        self.fixed.put(native, 0, 0);
    }
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize) {
        self.window.move_(x as i32, y as i32);
        self.window.resize(w as i32, h as i32);
    }
    fn move_child(&mut self, widget: &dyn gui::Widget, x: isize, y: isize, w: isize, h: isize) {
        let native_any = widget.get_native();
        let native: &gtk::Widget = native_any.as_ref().downcast_ref().unwrap();
        native.set_size_request(w as i32, h as i32);
        self.fixed.move_(native, x as i32, y as i32);
    }
}

impl gui::Widget for Window{
    fn upcast(&self) -> Box<dyn gui::Widget> {
        Box::new(self.clone())
    }
    fn get_native(&self) -> Box<dyn Any> {
        Box::new(self.window.clone())
    }
    fn set_hidden(&mut self, hidden: bool) {
        if hidden {
            self.window.show();
        } else {
            self.window.hide();
        }
    }
    fn resize(&mut self, width: isize, height: isize) {
        self.window.resize(width as i32, height as i32);
    }
}