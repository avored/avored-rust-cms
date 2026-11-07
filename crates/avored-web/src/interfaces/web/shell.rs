use crate::interfaces::web::web_routes::WebApp;
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Shell() -> impl IntoView {
    // log::info!("Shell component mounted");

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <link rel="icon" type="image/x-icon" href="/assets/images/favicon.ico"/>
                <Stylesheet id="leptos" href={format!("/assets/css/app.css?v={}", chrono::Utc::now().timestamp())} />
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <MetaTags/>
            </head>
            <body>
                <WebApp/>
                <script type="module" src={format!("/assets/js/app.js?v={}", chrono::Utc::now().timestamp())}></script>
                <script type="module">
                    {r#"import init, { hydrate } from '/public/pkg/avored-rust-cms.js';
                    await init('/public/pkg/avored-rust-cms.wasm').then(hydrate);
                    window.leptos_hydrated = true;
                    console.log('Leptos hydrated');
                    window.dispatchEvent(new CustomEvent('leptos-hydrated'));"#}
                </script>
            </body>
        </html>
    }
}
