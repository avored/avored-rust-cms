// use leptos_meta::MetaTags;
// use leptos::prelude::*;


// use crate::pages::app::App;

// pub fn shell(options: LeptosOptions) -> impl IntoView {
//     view! {
//         <!DOCTYPE html>
//         <html lang="en">
//             <head>
//                 <meta charset="utf-8"/>
//                 <meta name="viewport" content="width=device-width, initial-scale=1"/>
//                 <AutoReload options=options.clone() />
//                 <HydrationScripts options/>
//                 <MetaTags/>
//             </head>
//             <body>
//                 <App/>
//             </body>
//         </html>
//     }
// }




use leptos::prelude::*;
use leptos_meta::*;
use crate::pages::app::App;

#[component]
pub fn Shell() -> impl IntoView {

    // log::info!("Shell component mounted");

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <link rel="icon" type="image/x-icon" href="/public/images/favicon.ico"/>
                <Stylesheet id="leptos" href="/public/css/app.css"/>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script type="module" src="/public/js/app.js"></script>
                <MetaTags/>
            </head>
            <body>
                <App/>
                <script type="module">
                    {r#"import init, { hydrate } from '/pkg/new-avored.js';
                    init('/pkg/new-avored.wasm').then(hydrate);"#}
                </script>
            </body>
        </html>
    }
}
