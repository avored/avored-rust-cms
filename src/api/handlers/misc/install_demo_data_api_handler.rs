use std::collections::BTreeMap;
use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
};
use axum::{extract::State, Json, response::IntoResponse, Extension};
use serde::Serialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::models::token_claim_model::LoggedInUser;

pub async fn install_demo_data_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - install_demo_data_api_handler", "HANDLER");

    let sql = "
        CREATE pages:wvb4100904eaf3ykz64c CONTENT {
            name: 'Home Page',
            identifier: 'home-page',
            page_fields: [
                {
                    name: 'hero page title',
                    identifier: 'hero-page-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'AvoRed Rust Content Management System'
                },
                {
                    name: 'hero page description',
                    identifier: 'hero-page-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Avored rust Content Management System (CMS) is user-friendly software that enables effortless creation, management, and modification of digital content on websites, empowering users to maintain an effective online presence without technical skills.'
                },
                {
                    name: 'get started button title',
                    identifier: 'get-started-button-title',
                    data_type: 'text',
                    field_type: 'TEXT',field_content: 'Get started'
                },
                {
                    name: 'get started button url',
                    identifier: 'get-started-button-url',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'https://github.com/avored/avored-rust-cms'
                },
                {
                    name: 'demo button title',
                    identifier: 'demo-button-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Live demo'
                },
                {
                    name: 'demo button url',
                    identifier: 'demo-button-url',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'https://demo.avored.com'
                },
                {
                    name: 'rate us number',
                    identifier: 'rate-us-number',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: '68'
                },
                {
                    name: 'rate us title',
                    identifier: 'rate-us-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Rate us'
                },
                {
                    name: 'rate us description',
                    identifier: 'rate-us-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Help us by giving a star on GitHub. Spread the word by recommending AvoRed to your network and help to get the better product.'
                },
                {
                    name: 'commit number',
                    identifier: 'commit-number',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: '730'
                },
                {
                    name: 'commit title',
                    identifier: 'commit-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Commit'
                },
                {
                    name: 'commit description',
                    identifier: 'commit-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Our project has over 500 commits, showcasing our teams dedication. Each commit enhances features, fixes bugs, and improves performance, ensuring a high-quality software product.'
                },
                {
                    name: 'contribute number',
                    identifier: 'contribute-number',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: '9'
                },
                {
                    name: 'contribute title',
                    identifier: 'contribute-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Contribute'
                },
                {
                    name: 'contribute description',
                    identifier: 'contribute-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Wish to contribute, AvoRed is 100% free and open-source under the GPL-3.0 license. Fork it on GitHub and help make it better.'
                },

                {
                    name: 'Content modeling title',
                    identifier: 'content-modeling-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Content Modeling'
                },
                {
                    name: 'Content modeling description',
                    identifier: 'content-modeling-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'A flexible and customizable content modeling system that allows users to define content structures (schemas) and relationships between different content types. This enables the creation of structured content that can be reused across various platforms and channels.'
                },
                {
                    name: 'Api-first approach title',
                    identifier: 'api-first-approach-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'API-first Approach'
                },
                {
                    name: 'Api-first approach description',
                    identifier: 'api-first-approach-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Built with an API-first architecture, allowing content to be accessed and delivered via APIs (RESTful or GraphQL). This decoupled approach enables content to be distributed to any device or platform, facilitating omnichannel content delivery.'
                },
                {
                    name: 'Content versioning and history title',
                    identifier: 'content-versioning-and-history-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Content Versioning and History'
                },
                {
                    name: 'Content versioning and history description',
                    identifier: 'content-versioning-and-history-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Support for content versioning and revision history, allowing users to track changes, revert to previous versions, and collaborate effectively on content creation and updates.'
                },
                {
                    name: 'Scalability and performance title',
                    identifier: 'scalability-and-performance-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Scalability and Performance'
                },
                {
                    name: 'Scalability and performance description',
                    identifier: 'scalability-and-performance-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Designed to handle large volumes of content and high traffic efficiently, with features such as caching, CDN (Content Delivery Network) support, and scalability options to ensure optimal performance across diverse environments.'
                },
                {
                    name: 'Integration capabilities title',
                    identifier: 'integration-capabilities-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Integration Capabilities'
                },
                {
                    name: 'Integration capabilities description',
                    identifier: 'integration-capabilities-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Extensive integration capabilities with third-party services, tools, and frameworks through webhooks, plugins, or custom integrations. This allows seamless connectivity with other systems such as eCommerce platforms, CRM systems, analytics tools, and more.'
                },
                {
                    name: 'Content localization and internationalization title',
                    identifier: 'content-localization-and-internationalization-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Content Localization and Internationalization'
                },
                {
                    name: 'Content localization and internationalization description',
                    identifier: 'content-localization-and-internationalization-description',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Capabilities to manage multilingual and localized content efficiently, including tools for translating content, managing language variations, and adapting content for different regions or markets.'
                },

                {
                    name: 'Contact us title',
                    identifier: 'contact-us-title',
                    data_type: 'text',
                    field_type: 'TEXT',
                    field_content: 'Need help developing content management solution with avored?'
                },
                {
                    name: 'Contact us description',
                    identifier: 'contact-us-description',
                    data_type: 'textarea',
                    field_type: 'TEXT',
                    field_content: 'We had love to talk with you about your cms projects whether it is a brand new website or a rework of your existing one. We would happily answer any questions you may have about developing with avored. Do not have a development team?Do not worry, we can implement the avored solution for you according to your business requirements. We can surely help you achieve your goals with a customized avored design. For a FREE consultation with our expert development team, simply leave your details below and we will get back to you soon.'
                },
            ],
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };
    ";

    let vars = BTreeMap::from([
        ("email".into(), logged_in_user.email.into()),
    ]);

    let (ds, ses) = &state.db;

    let responses = ds.execute(sql, ses, Some(vars)).await?;

    println!("{responses:?}");
    println!();

    let mut file = File::create("public/install_demo").await?;
    file.write_all(b".gitkeep").await?;

    let response = DemoDataViewModel {
        status: true
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct DemoDataViewModel {
    pub status: bool,
}
