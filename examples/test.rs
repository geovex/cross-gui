extern crate cross_gui;
//use cross_gui::gui;

fn main() {
    let mut g = cross_gui::gui::default_gui();
    let mut essential_window = g.new_window();
    essential_window.set_title("essential");
    essential_window.move_(30, 30, 200, 400);
    let mut non_essential_window = g.new_window();
    non_essential_window.set_title("nonessential");
    non_essential_window.move_(330, 330, 200, 400);
    non_essential_window.set_exit_on_close(false);
    let mut button = g.new_button();
    button.set_title("button");
    essential_window.add_widget(button.upcast());
    button.move_(110, 10, 80, 40);
    let mut label = g.new_label();
    essential_window.add_widget(label.upcast());
    label.set_title("label");
    label.move_(0, 0, 90, 40);
    g.event_loop();
}
