use std::collections::HashMap;

use nwg::{Window, bind_event_handler,Event, EventData, ControlHandle, Button, ButtonBuilder};

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
fn window_callbacks(event: Event,event_data:EventData,handle:ControlHandle) {
    if event==Event::OnWindowClose {
        nwg::stop_thread_dispatch();
    }
}
fn main() {
    nwg::init();
    let mut manager_window: Window=Window::default();
    nwg::Window::builder().title("Virtual Computer Manager").size((500,500)).build(&mut manager_window);
    nwg::dispatch_thread_events();
}
