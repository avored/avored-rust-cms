use crate::pages::app_layout::AppLayout;
use crate::pages::auth::login_page::LoginPage;
use crate::pages::dashboard::dashboard_page::DashboardPage;
use crate::pages::{
    home_page::HomePage,
    not_found::NotFound,
    protected_routes::{provide_auth_context, ProtectedRoute},
};
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::*;
use leptos_router::*;
use leptos_router::{
    components::{Route, Router, Routes},
};

#[derive(Clone)]
pub struct AppUserContext {
    pub token: String,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_auth_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet


        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/auth/login") view=LoginPage/>

                    <ParentRoute path=path!("/admin") view=|| view! {
                         <ProtectedRoute
                             fallback=std::sync::Arc::new(|| view! { "Redirecting..." }.into_any())
                         >
                            <AppLayout/>
                         </ProtectedRoute>
                    }>
                        <Route path=path!("/dashboard") view=DashboardPage/>
                    </ParentRoute>

                    <Route path=path!("/*any") view=NotFound/>

                </Routes>
            </main>
        </Router>
    }
}
