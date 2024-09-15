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
         CREATE components CONTENT {
            name: 'Hero component',
            identifier: 'hero-component',
            elements: [
                {
                    name: 'hero page title',
                    identifier: 'hero-page-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },{
                    name: 'hero page description',
                    identifier: 'hero-page-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },{
                    name: 'get started button title',
                    identifier: 'get-started-button-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },{
                    name: 'get started button url',
                    identifier: 'get-started-button-url',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },{
                    name: 'demo button title',
                    identifier: 'demo-button-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },{
                    name: 'demo button url',
                    identifier: 'demo-button-url',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                }
            ,],
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now(),
        };

        CREATE components CONTENT {
            name: 'Repository component',
            identifier: 'repository-component',
            elements: [
                {
                    name: 'Rate us number',
                    identifier: 'rate-us-number',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Rate us title',
                    identifier: 'rate-us-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Rate us description',
                    identifier: 'rate-us-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Commit number',
                    identifier: 'commit-number',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Commit title',
                    identifier: 'commit-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Commit description',
                    identifier: 'commit-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Contribute number',
                    identifier: 'contribute-number',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Contribute title',
                    identifier: 'contribute-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Contribute description',
                    identifier: 'contribute-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                }
            ,],
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now(),
        };

        CREATE components CONTENT {
            name: 'Key features component',
            identifier: 'key-features-component',
            elements: [
                {
                    name: 'Content modeling title',
                    identifier: 'content-modeling-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Content modeling description',
                    identifier: 'content-modeling-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Api-first approach title',
                    identifier: 'api-first-approach-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Api-first approach description',
                    identifier: 'api-first-approach-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Content versioning and history title',
                    identifier: 'content-versioning-and-history-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Content versioning and history description',
                    identifier: 'content-versioning-and-history-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Scalability and performance title',
                    identifier: 'scalability-and-performance-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Scalability and performance description',
                    identifier: 'Scalability and performance-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Integration capabilities title',
                    identifier: 'integration-capabilities-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Integration capabilities description',
                    identifier: 'integration-capabilities-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Content localization and internationalization title',
                    identifier: 'content-localization-and-internationalization-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
                {
                    name: 'Content localization and internationalization description',
                    identifier: 'content-localization-and-internationalization-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                }
            ,],
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now(),
        };

        CREATE components CONTENT {
            name: 'Contact us component',
            identifier: 'contact-us-component',
            elements: [
                {
                    name: 'Contact us title',
                    identifier: 'contact-us-title',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },{
                    name: 'Contact us description',
                    identifier: 'contact-us-description',
                    element_type: 'text',
                    element_data_type: 'TEXT',
                    element_data: [],
                },
            ],
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now(),
        };

        CREATE pages:wvb4100904eaf3ykz64c CONTENT {
            name: 'Home Page',
            identifier: 'home-page',
            components_content: [
                {
                    name: 'Hero component',
                    identifier: 'hero-component',
                    elements: [
                        {
                            name: 'hero page title',
                            identifier: 'hero-page-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'AvoRed Rust Content Management System'
                        },
                        {
                            name: 'hero page description',
                            identifier: 'hero-page-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Avored rust Content Management System (CMS) is user-friendly software that enables effortless creation, management, and modification of digital content on websites, empowering users to maintain an effective online presence without technical skills.'
                        },
                        {
                            name: 'get started button title',
                            identifier: 'get-started-button-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Get started'
                        },
                        {
                            name: 'get started button url',
                            identifier: 'get-started-button-url',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'https://github.com/avored/avored-rust-cms'
                        },
                        {
                            name: 'demo button title',
                            identifier: 'demo-button-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Live demo'
                        },
                        {
                            name: 'demo button url',
                            identifier: 'demo-button-url',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'https://demo.avored.com'
                        },
                    ]
                },{
                    name: 'Repository component',
                    identifier: 'repository-component',
                    elements: [
                        {
                            name: 'rate us number',
                            identifier: 'rate-us-number',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: '68'
                        },
                        {
                            name: 'rate us title',
                            identifier: 'rate-us-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Rate us'
                        },
                        {
                            name: 'rate us description',
                            identifier: 'rate-us-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Help us by giving a star on GitHub. Spread the word by recommending AvoRed to your network and help to get the better product.'
                        },
                        {
                            name: 'commit number',
                            identifier: 'commit-number',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: '730'
                        },
                        {
                            name: 'commit title',
                            identifier: 'commit-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Commit'
                        },
                        {
                            name: 'commit description',
                            identifier: 'commit-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Our project has over 500 commits, showcasing our teams dedication. Each commit enhances features, fixes bugs, and improves performance, ensuring a high-quality software product.'
                        },
                        {
                            name: 'contribute number',
                            identifier: 'contribute-number',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: '9'
                        },
                        {
                            name: 'contribute title',
                            identifier: 'contribute-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Contribute'
                        },
                        {
                            name: 'contribute description',
                            identifier: 'contribute-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Wish to contribute, AvoRed is 100% free and open-source under the GPL-3.0 license. Fork it on GitHub and help make it better.'
                        },
                    ]
                }, {
                    name: 'Key features component',
                    identifier: 'key-features-component',
                    elements: [
                        {
                            name: 'Content modeling title',
                            identifier: 'content-modeling-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Content Modeling'
                        },
                        {
                            name: 'Content modeling description',
                            identifier: 'content-modeling-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'A flexible and customizable content modeling system that allows users to define content structures (schemas) and relationships between different content types. This enables the creation of structured content that can be reused across various platforms and channels.'
                        },
                        {
                            name: 'Api-first approach title',
                            identifier: 'api-first-approach-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'API-first Approach'
                        },
                        {
                            name: 'Api-first approach description',
                            identifier: 'api-first-approach-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Built with an API-first architecture, allowing content to be accessed and delivered via APIs (RESTful or GraphQL). This decoupled approach enables content to be distributed to any device or platform, facilitating omnichannel content delivery.'
                        },
                        {
                            name: 'Content versioning and history title',
                            identifier: 'content-versioning-and-history-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Content Versioning and History'
                        },
                        {
                            name: 'Content versioning and history description',
                            identifier: 'content-versioning-and-history-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Support for content versioning and revision history, allowing users to track changes, revert to previous versions, and collaborate effectively on content creation and updates.'
                        },
                        {
                            name: 'Scalability and performance title',
                            identifier: 'scalability-and-performance-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Scalability and Performance'
                        },
                        {
                            name: 'Scalability and performance description',
                            identifier: 'scalability-and-performance-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Designed to handle large volumes of content and high traffic efficiently, with features such as caching, CDN (Content Delivery Network) support, and scalability options to ensure optimal performance across diverse environments.'
                        },
                        {
                            name: 'Integration capabilities title',
                            identifier: 'integration-capabilities-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Integration Capabilities'
                        },
                        {
                            name: 'Integration capabilities description',
                            identifier: 'integration-capabilities-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Extensive integration capabilities with third-party services, tools, and frameworks through webhooks, plugins, or custom integrations. This allows seamless connectivity with other systems such as eCommerce platforms, CRM systems, analytics tools, and more.'
                        },
                        {
                            name: 'Content localization and internationalization title',
                            identifier: 'content-localization-and-internationalization-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Content Localization and Internationalization'
                        },
                        {
                            name: 'Content localization and internationalization description',
                            identifier: 'content-localization-and-internationalization-description',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Capabilities to manage multilingual and localized content efficiently, including tools for translating content, managing language variations, and adapting content for different regions or markets.'
                        },
                    ]
                }, {
                    name: 'Contact us component',
                    identifier: 'contact-us-component',
                    elements: [
                        {
                            name: 'Contact us title',
                            identifier: 'contact-us-title',
                            element_type: 'text',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'Need help developing content management solution with avored?'
                        },
                        {
                            name: 'Contact us description',
                            identifier: 'contact-us-description',
                            element_type: 'textarea',
                            element_data_type: 'TEXT',
                            element_data: [],
                            element_content: 'We had love to talk with you about your cms projects whether it is a brand new website or a rework of your existing one. We would happily answer any questions you may have about developing with avored. Do not have a development team?Do not worry, we can implement the avored solution for you according to your business requirements. We can surely help you achieve your goals with a customized avored design. For a FREE consultation with our expert development team, simply leave your details below and we will get back to you soon.'
                        },
                    ]
                }
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
