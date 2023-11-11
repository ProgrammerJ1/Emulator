use nwg::{Window, bind_event_handler,Event, EventData, ControlHandle};

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
fn window_callbacks(event: Event,event_data:EventData,handle:ControlHandle) {
    if event==Event::OnWindowClose {
        nwg::stop_thread_dispatch();
    }
}
fn main() {
    nwg::init();
    let mut builder=nwg::Window::builder();
    builder=builder.title("Virtual Computer Manager");
    builder=builder.size((500,500));
    let window:&mut Window=&mut Window::default();
    builder.build(window);
    window.set_visible(true);
    nwg::dispatch_thread_events();
    let window_handlers=bind_event_handler(&window.handle, &window.handle, window_callbacks);
}
