#![recursion_limit = "144"]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod interfaces;

rust_i18n::i18n!("../../resources/locales");

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::interfaces::web::web_routes::WebApp;

    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("failed to initialize browser logging");
    leptos::mount::hydrate_body(WebApp);
}
