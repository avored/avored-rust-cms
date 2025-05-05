use std::collections::BTreeMap;
use std::sync::Arc;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::admin_user::StoreAdminUserRequest;
use crate::api::proto::misc::{DeleteDemoDataRequest, DeleteDemoDataResponse, HealthCheckRequest, HealthCheckResponse, InstallDemoDataRequest, InstallDemoDataResponse, SetupRequest, SetupResponse};
use crate::api::proto::misc::misc_server::Misc;
use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::CreatableAdminUserModel;
use crate::models::role_model::CreatableRole;

pub struct MiscApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Misc for MiscApi {
    async fn setup(&self, request: Request<SetupRequest>) -> Result<Response<SetupResponse>, Status> {
        let req = request.into_inner();
        // let (valid, error_messages) = req.validate()?;

        // if !valid {
        //     return Err(Status::invalid_argument(error_messages))
        // }

        
        match self.
            state.
            misc_service.
            setup(
                req,
                &self.state.config.password_salt,
                &self.state.db
            ).await {
                Ok(reply) => Ok(Response::new(reply)),
                Err(e) => Err(Status::internal(e.to_string()))
            }

    }

    async fn health_check(&self, _request: Request<HealthCheckRequest>) -> Result<Response<HealthCheckResponse>, Status> {
        println!("request: {:?}", _request);
        let reply = HealthCheckResponse {
            status: true
        };

        println!("response: {:?}", reply);
        Ok(Response::new(reply))
    }
    
    async fn install_demo_data(
        &self, 
        _request: Request<InstallDemoDataRequest>
    ) -> Result<Response<InstallDemoDataResponse>, Status> {
        println!("request: {:?}", _request);

        let sql = "
        CREATE pages:wvb4100904eaf3ykz64c CONTENT {
            name: 'Home Page',
            identifier: 'home-page',
            content_fields: [
                {
                    name: 'hero page title',
                    identifier: 'hero-page-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'AvoRed Rust Content Management System' }
                },
                {
                    name: 'hero page description',
                    identifier: 'hero-page-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Avored rust Content Management System (CMS) is user-friendly software that enables effortless creation, management, and modification of digital content on websites, empowering users to maintain an effective online presence without technical skills.' }
                },
                {
                    name: 'get started button title',
                    identifier: 'get-started-button-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Get started' }
                },
                {
                    name: 'get started button url',
                    identifier: 'get-started-button-url',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'https://github.com/avored/avored-rust-cms' }
                },
                {
                    name: 'demo button title',
                    identifier: 'demo-button-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Live demo' }
                },
                {
                    name: 'demo button url',
                    identifier: 'demo-button-url',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'https://demo.avored.com' }
                },
                {
                    name: 'rate us number',
                    identifier: 'rate-us-number',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: '85' }
                },
                {
                    name: 'rate us title',
                    identifier: 'rate-us-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Rate us' }
                },
                {
                    name: 'rate us description',
                    identifier: 'rate-us-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Help us by giving a star on GitHub. Spread the word by recommending AvoRed to your network and help to get the better product.' }
                },
                {
                    name: 'commit number',
                    identifier: 'commit-number',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: '856' }
                },
                {
                    name: 'commit title',
                    identifier: 'commit-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Commit' }
                },
                {
                    name: 'commit description',
                    identifier: 'commit-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Our project has over 500 commits, showcasing our teams dedication. Each commit enhances features, fixes bugs, and improves performance, ensuring a high-quality software product.' }
                },
                {
                    name: 'contribute number',
                    identifier: 'contribute-number',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: '10' }
                },
                {
                    name: 'contribute title',
                    identifier: 'contribute-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Contribute' }
                },
                {
                    name: 'contribute description',
                    identifier: 'contribute-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Wish to contribute, AvoRed is 100% free and open-source under the GPL-3.0 license. Fork it on GitHub and help make it better.' }
                },

                {
                    name: 'Content modeling title',
                    identifier: 'content-modeling-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Content Modeling' }
                },
                {
                    name: 'Content modeling description',
                    identifier: 'content-modeling-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'A flexible and customizable content modeling system that allows users to define content structures (schemas) and relationships between different content types. This enables the creation of structured content that can be reused across various platforms and channels.' }
                },
                {
                    name: 'Api-first approach title',
                    identifier: 'api-first-approach-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'API-first Approach' }
                },
                {
                    name: 'Api-first approach description',
                    identifier: 'api-first-approach-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Built with an API-first architecture, allowing content to be accessed and delivered via APIs (RESTful or GraphQL). This decoupled approach enables content to be distributed to any device or platform, facilitating omnichannel content delivery.' }
                },
                {
                    name: 'Content versioning and history title',
                    identifier: 'content-versioning-and-history-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Content Versioning and History' }
                },
                {
                    name: 'Content versioning and history description',
                    identifier: 'content-versioning-and-history-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Support for content versioning and revision history, allowing users to track changes, revert to previous versions, and collaborate effectively on content creation and updates.' }
                },
                {
                    name: 'Scalability and performance title',
                    identifier: 'scalability-and-performance-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Scalability and Performance' }
                },
                {
                    name: 'Scalability and performance description',
                    identifier: 'scalability-and-performance-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Designed to handle large volumes of content and high traffic efficiently, with features such as caching, CDN (Content Delivery Network) support, and scalability options to ensure optimal performance across diverse environments.' }
                },
                {
                    name: 'Integration capabilities title',
                    identifier: 'integration-capabilities-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Integration Capabilities' }
                },
                {
                    name: 'Integration capabilities description',
                    identifier: 'integration-capabilities-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Extensive integration capabilities with third-party services, tools, and frameworks through webhooks, plugins, or custom integrations. This allows seamless connectivity with other systems such as eCommerce platforms, CRM systems, analytics tools, and more.' }
                },
                {
                    name: 'Content localization and internationalization title',
                    identifier: 'content-localization-and-internationalization-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Content Localization and Internationalization' }
                },
                {
                    name: 'Content localization and internationalization description',
                    identifier: 'content-localization-and-internationalization-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Capabilities to manage multilingual and localized content efficiently, including tools for translating content, managing language variations, and adapting content for different regions or markets.' }
                },

                {
                    name: 'Contact us title',
                    identifier: 'contact-us-title',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'Need help developing content management solution with avored?' }
                },
                {
                    name: 'Contact us description',
                    identifier: 'contact-us-description',
                    data_type: 'TEXT',
                    field_type: 'TEXT',
                    field_content: { text_value: 'We had love to talk with you about your cms projects whether it is a brand new website or a rework of your existing one. We would happily answer any questions you may have about developing with avored. Do not have a development team?Do not worry, we can implement the avored solution for you according to your business requirements. We can surely help you achieve your goals with a customized avored design. For a FREE consultation with our expert development team, simply leave your details below and we will get back to you soon.' }
                },
            ],
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };
    ";

        let logged_in_user = "admin@admin.com";
        let vars = BTreeMap::from([("email".into(), logged_in_user.into())]);

        let (ds, ses) = &self.state.db;

        let responses = ds.execute(sql, ses, Some(vars)).await.unwrap();

        println!("{responses:?}");
        println!();

        let mut file = File::create("public/install_demo").await.unwrap();
        file.write_all(b".gitkeep").await?;

        // @todo create a demo role
        // @todo create a demo visitor user

        let demo_role = CreatableRole {
            name: "Demo visitor role".to_string(),
            identifier: "demo-visitor-role".to_string(),
            logged_in_username: logged_in_user.clone().to_string(),
            permissions: vec![
                String::from("dashboard"),
                String::from("get_setting"),
                String::from("page_table"),
                String::from("page_create"),
                String::from("page_edit"),
                String::from("get_page"),
                String::from("page_delete"),
                String::from("asset_table"),
                String::from("asset_create"),
                String::from("asset_edit"),
                String::from("asset_delete"),
            ],
        };

        let created_role_model = self
            .state
            .admin_user_service
            .store_role(demo_role, &self.state.db).await?;

        // let password = "admin123".as_bytes();
        // let salt = SaltString::from_b64(&self.state.config.password_salt)?;

        // let argon2 = Argon2::default();
        // let password_hash = argon2
        //     .hash_password(password, &salt)
        //     .expect("Error occurred while encrypted password")
        //     .to_string();

        let creatable_admin_user = StoreAdminUserRequest {
            full_name: "Demo admin user".to_string(),
            email: "demo@avored.com".to_string(),
            password: "admin123".to_string(),
            confirm_password: "admin123".to_string(),
            is_super_admin: false,
            profile_image_content: vec![],
            profile_image_file_name: "".to_string(),
        };

        
        let created_admin_user = self.state
            .admin_user_service
            .store(
                creatable_admin_user, 
                "admin@admin.com".to_string(), 
                &self.state.config.password_salt, 
                &self.state.db
            )
            .await?;

        println!("Created admin user: {:?}", created_admin_user);
        
        let reply = InstallDemoDataResponse {};

        println!("response: {:?}", reply);
        Ok(Response::new(reply))   
    }
    
    async fn delete_demo_data(
        &self,
        _request: Request<DeleteDemoDataRequest>
    ) -> Result<Response<DeleteDemoDataResponse>, Status> {

        let sql = "
            DELETE roles WHERE identifier='demo-visitor-role';
            DELETE admin_users WHERE email='demo@avored.com';
            DELETE pages WHERE identifier='home-page';
        ";
    
        // let vars = BTreeMap::from([("email".into(), logged_in_user.email.into())]);
    
        let (ds, ses) = &self.state.db;
    
        let responses = ds.execute(sql, ses, None).await.unwrap();

        println!("{responses:?}");
        println!();

        tokio::fs::remove_file("public/install_demo").await?;
        let reply = DeleteDemoDataResponse {};

        println!("response: {:?}", reply);
        Ok(Response::new(reply))
    }
}
