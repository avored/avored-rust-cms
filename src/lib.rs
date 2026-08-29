pub mod pages;

#[cfg(feature = "ssr")]
pub mod error;

#[cfg(feature = "ssr")]
pub mod api;

#[cfg(feature = "ssr")]
pub mod providers;


#[cfg(feature = "ssr")]
pub mod avored_state;



#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    
    use crate::pages::app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
