pub mod app;
pub mod helpers;
pub mod layout;
pub mod widgets;

pub fn spawn_debug_thread() {
    app::start();
}
