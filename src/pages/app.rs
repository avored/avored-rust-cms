use leptos::{prelude::*};
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::pages::home_page::HomePage;
use crate::pages::auth::login_page::LoginPage;
use crate::pages::dashboard::dashboard_page::DashboardPage;


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("/login") view=LoginPage/>
                    <Route path=StaticSegment("/dashboard") view=DashboardPage/>
                </Routes>
            </main>
        </Router>
    }
}
