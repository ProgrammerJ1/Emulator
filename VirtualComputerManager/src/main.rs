extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
use nwg::{Window, bind_event_handler,Event, EventData, ControlHandle};
fn window_callbacks(event: Event,_event_data:EventData,_handle:ControlHandle) {
    if event==Event::OnWindowClose {
        nwg::stop_thread_dispatch();
    }
}
fn main() {
    let _=nwg::init();
    let mut manager_window: Window=Window::default();
    let _=nwg::Window::builder().title("Virtual Computer Manager").size((500,500)).build(&mut manager_window);
    bind_event_handler(&manager_window.handle, &manager_window.handle, window_callbacks);
    nwg::dispatch_thread_events();
}
