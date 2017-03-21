use gtk;
use std::marker::PhantomData;
use ::gui::Gui;

pub struct PalGui<M> {
    phantom: PhantomData<M>
}

impl<M> Gui for PalGui<M> {
    type Window = ();
    type Msg = M;
    fn new() -> PalGui<M> {
        PalGui {
            phantom: PhantomData
        }
    }
    fn new_window(&mut self) -> () {
        unimplemented!()
    }
    fn event_loop(&mut self) {
        gtk::main();
    }
    fn send_msg(&mut self, _msg: M) {
        unimplemented!()
    }
}