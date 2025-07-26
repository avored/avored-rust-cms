use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use crate::pages::home_page::HomePage;
use crate::pages::admin::auth::login_page::LoginPage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found">
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/admin/login") view=LoginPage />
            //     <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </Router>
    }
}
