pub mod interfaces;

#[cfg(feature = "ssr")]
pub mod core;

#[cfg(feature = "ssr")]
pub mod infrastructure;

#[cfg(feature = "ssr")]
pub mod error;

#[cfg(feature = "ssr")]
pub mod api;

#[cfg(feature = "ssr")]
pub mod providers;

#[cfg(feature = "ssr")]
pub mod avored_state;


rust_i18n::i18n!("resources/locales");


#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    
    use crate::interfaces::web::web_routes::WebApp;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(WebApp);
}
