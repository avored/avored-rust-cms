use crate::app::App;

mod app;
mod pages;
mod layouts;
mod components;

fn main() {
     _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
