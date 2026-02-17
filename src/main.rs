
#[cfg(feature = "ssr")]
use new_avored::error::Result;


#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<()>{
    use new_avored::infra::setup::init_app_state;
    use new_avored::infra::app::create_app;

    let state = init_app_state().await?;
    // let conf = get_configuration(None).unwrap();
    // let addr = &state.leptos_options.site_addr;

    let app = create_app(state).await?;

    // run our app with hyper
    println!("listening on http://{}", 3000);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
