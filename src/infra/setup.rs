use axum::extract::FromRef;
use leptos::config::{LeptosOptions, get_configuration};
use crate::{error::Result};

pub async fn init_app_state() -> Result<AppState> {
    let conf = get_configuration(None)?;
    // let addr = conf.leptos_options.site_addr;

    Ok(AppState {
        leptos_options: conf.leptos_options,
    })
}



#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}


impl FromRef<AppState> for LeptosOptions {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.leptos_options.clone()
    }
}
