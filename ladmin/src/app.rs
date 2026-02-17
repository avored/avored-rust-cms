use leptos::task::spawn_local;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ladmin.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);

    let (response, set_response) =
        signal(String::from("Click the button to call gRPC service"));
    let (loading, set_loading) = signal(false);


    let call_grpc = move |_| {
        set_loading.set(true);
        set_response.set("Calling gRPC service...".to_string());

        spawn_local(async move {
            // Call gRPC service (update URL to your actual gRPC-Web server)

            use tonic_web_wasm_client::Client;
            use crate::proto::misc::misc_client::MiscClient;
            use crate::proto::misc::HealthCheckRequest;

            let base_url = "http://localhost:50051";
            let mut misc_client = MiscClient::new(Client::new(base_url.to_string()));
            let request = HealthCheckRequest {};

            let response =  misc_client.health_check(request)
                .await.expect("success response").into_inner();

                if response.status {
                    set_response.set("all fine".to_string());
                }
            
            set_loading.set(false);
        });
        
    };
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to grpc!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>

        <h1>"gRPC-Web Demo"</h1>
        <p>"This demonstrates calling a gRPC service from Leptos WASM"</p>
        <button on:click=call_grpc disabled=move || loading.get()>
            {move || if loading.get() { "Loading..." } else { "Call gRPC Service" }}
        </button>
        <p>{move || response.get()}</p>
    }
}
