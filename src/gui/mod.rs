use ::pal;

pub fn default_gui<Msg:Send>() -> pal::PalGui<Msg> {
    pal::PalGui::new()
}

pub trait Gui {
    type Window;
    type Msg;
    fn new() -> Self;
    fn new_window(&mut self) -> Self::Window;
    fn event_loop(&mut self);
    fn send_msg(&mut self, msg: Self::Msg);
}

pub trait Window {
}