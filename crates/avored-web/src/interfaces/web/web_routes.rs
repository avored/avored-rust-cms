use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};

use crate::interfaces::web::pages::admin::api_key::{
    ApiKeyCreatePage, ApiKeyEditPage, ApiKeyIndexPage,
};
use crate::interfaces::web::pages::admin::collections::{
    collection_create_page::CollectionCreatePage, collection_edit_page::CollectionEditPage,
    collection_index_page::CollectionIndexPage,
};
use crate::interfaces::web::pages::admin::email::{
    EmailTemplateCreatePage, EmailTemplateEditPage, EmailTemplateIndexPage,
};
use crate::interfaces::web::pages::admin::entity::{
    EntityCreatePage, EntityEditPage, EntityIndexPage,
};
use crate::interfaces::web::pages::admin::profile::AdminProfilePage;
use crate::interfaces::web::pages::admin::setting::SettingPage;
use crate::interfaces::web::pages::admin::workflow::{
    WorkflowCreatePage, WorkflowEditPage, WorkflowIndexPage,
};
use crate::interfaces::web::pages::misc::setup_page::SetupPage;
use crate::interfaces::web::pages::{
    auth::login_page::LoginPage, dashboard_page::DashboardPage, home_page::HomePage,
    layouts::app_layout::AppLayout,
};
use crate::interfaces::web::protected_routes::provide_auth_context;
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
                    <Route path=path!("/setup") view=SetupPage/>

                    <ParentRoute path=path!("/admin") view=AppLayout>
                        <Route path=StaticSegment("") view=DashboardPage/>
                        <Route path=path!("profile") view=AdminProfilePage/>
                        <Route path=path!("entity") view=EntityIndexPage/>
                        <Route path=path!("entity/create") view=EntityCreatePage/>
                        <Route path=path!("entity/:id/edit") view=EntityEditPage/>
                        <Route path=path!("collections") view=CollectionIndexPage/>
                        <Route path=path!("collections/create") view=CollectionCreatePage/>
                        <Route path=path!("collections/:id/edit") view=CollectionEditPage/>
                        <Route path=path!("email-templates") view=EmailTemplateIndexPage/>
                        <Route path=path!("email-templates/create") view=EmailTemplateCreatePage/>
                        <Route path=path!("email-templates/:id/edit") view=EmailTemplateEditPage/>
                        <Route path=path!("workflows") view=WorkflowIndexPage/>
                        <Route path=path!("workflows/create") view=WorkflowCreatePage/>
                        <Route path=path!("workflows/:id/edit") view=WorkflowEditPage/>
                        <Route path=path!("api-keys") view=ApiKeyIndexPage/>
                        <Route path=path!("api-keys/create") view=ApiKeyCreatePage/>
                        <Route path=path!("api-keys/:id/edit") view=ApiKeyEditPage/>
                        <Route path=path!("settings") view=SettingPage/>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}
