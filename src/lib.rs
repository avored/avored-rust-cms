pub mod pages;



#[cfg(feature = "ssr")]
pub mod error;

#[cfg(feature = "ssr")]
pub mod infra;


#[cfg(feature = "ssr")]
pub mod grpc_server;

#[cfg(feature = "ssr")]
pub mod server {
    // This will allow us to use it in main.rs if needed,
    // or just leave it in lib.rs
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::pages::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
