use gtk::Window as GtkWindow;
use gtk::WidgetExt as _;
use gtk::GtkWindowExt as _;
pub struct Window {
    inner: GtkWindow,
}

impl Window {
    pub fn new() -> Window {
        let mut gtk_window = GtkWindow::new(gtk::WindowType::Toplevel);
        gtk_window.show();
        Window { inner: gtk_window }
    }
}

impl ::gui::Window for Window {
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
}
