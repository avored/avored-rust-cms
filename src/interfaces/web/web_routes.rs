use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};

use crate::interfaces::web::pages::{
    auth::login_page::LoginPage, dashboard_page::DashboardPage, home_page::HomePage,
    layouts::app_layout::AppLayout,
};
use crate::interfaces::web::protected_routes::{provide_auth_context, ProtectedRoute};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path, StaticSegment,
};

#[component]
pub fn WebApp() -> impl IntoView {
    provide_auth_context();
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/public/pkg/avored-rust-cms.css"/>
        <Title text="Avored CMS"/>

        <Router>
            <main>
            <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("") view=HomePage/>
                    <Route path=path!("/auth/login") view=LoginPage/>

                    <ParentRoute path=path!("/admin") view= move || view! {
                        <ProtectedRoute
                            fallback=|| view! { "Redirecting..." }.into_any()
                            children=|| vec![view! { <AppLayout/> }.into_any()]
                        />
                    }>
                        <Route path=StaticSegment("") view=DashboardPage/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
