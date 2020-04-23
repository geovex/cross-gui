use gtk::GtkWindowExt as _;
use gtk::WidgetExt as _;
use gtk::Window as GtkWindow;
use std::{cell::RefCell, rc::Rc};
pub struct Window {
    inner: GtkWindow,
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
        let gtk_window = GtkWindow::new(gtk::WindowType::Toplevel);
        let user_data = Rc::new(RefCell::new(UserData::new()));
        let user_data_clone = user_data.clone();
        gtk_window.connect_destroy(move |_| {
            if user_data_clone.borrow().exit_on_close {
                gtk::main_quit()
            }
        });
        gtk_window.show();
        Window {
            user_data,
            inner: gtk_window,
        }
    }
}

impl ::gui::Window for Window {
    fn set_title(&mut self, title: &str) {
        self.inner.set_title(title);
    }
    fn set_hidden(&mut self, hidden: bool) {
        if hidden {
            self.inner.show();
        } else {
            self.inner.hide();
        }
    }
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize) {
        self.inner.move_(x as i32, y as i32);
        self.inner.resize(w as i32, h as i32);
    }
    fn set_exit_on_close(&mut self, exit: bool) {
        self.user_data.borrow_mut().exit_on_close = exit;
    }
}
