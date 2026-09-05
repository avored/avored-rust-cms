#[cfg(feature = "ssr")]
use avored_rust_cms::error::Result;
#[cfg(feature = "ssr")]
use avored_rust_cms::infra::app::create_app;
#[cfg(feature = "ssr")]
use avored_rust_cms::infra::setup::init_app_state;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<()> {
    let state = init_app_state().await?;
    let app = create_app(state).await?;

    println!("listening on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
}
