use nwg::Window;

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
fn main() {
    nwg::init();
    let mut builder=nwg::Window::builder();
    builder=builder.title("Virtual Computer Manager");
    builder=builder.size((500,500));
    let window:&mut Window=&mut Window::default();
    builder.build(window);
    window.set_visible(true);
    nwg::dispatch_thread_events();
}
