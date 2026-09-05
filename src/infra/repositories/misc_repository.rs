use crate::domain::models::admin_user::StorableAdminUser;
use crate::error::Result;
use crate::infra::repositories::surreal_user_repository::SurrealUserRepository;
use async_trait::async_trait;
use surrealdb::types::{Datetime, Table, Value};

#[async_trait]
pub trait MiscRepository: Send + Sync {
    async fn setup(&self, storable_admin_user: StorableAdminUser) -> Result<bool>;
}

#[async_trait]
impl MiscRepository for SurrealUserRepository {
    async fn setup(&self, storable_admin_user: StorableAdminUser) -> Result<bool> {
        println!("Setting up database");
        let migrations_path = std::path::Path::new("./migrations");

        if !migrations_path.exists() {
            return Ok(false);
        }

        // @todo create a migrations table to track which migrations have been run, and only run new ones
        let mut entries = std::fs::read_dir(migrations_path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<std::result::Result<Vec<_>, std::io::Error>>()?;

        entries.sort();

        for path in entries {
            if path.extension().and_then(|s| s.to_str()) == Some("surql") {
                println!("Executing migration: {:?}", path);
                let content = std::fs::read_to_string(&path)?;
                self.db.query(&content).await?;
            }
        }

        let sql = "CREATE $table CONTENT {
            full_name: $full_name,
            email: $email,
            password_hash: $password_hash,
            profile_image: $profile_image,
            is_super_admin: $is_super_admin,
            created_by: $created_by,
            updated_by: $updated_by,
            created_at: $created_at,
            updated_at: $updated_at,
        }";


        let mut create_admin_user_response = self
            .db
            .query(sql)
            .bind(("table", Table::from("admin_users")))
            .bind(("full_name", storable_admin_user.full_name))
            .bind(("email", storable_admin_user.email))
            .bind(("password_hash", storable_admin_user.password_hash))
            .bind(("profile_image", storable_admin_user.profile_image))
            .bind(("is_super_admin", storable_admin_user.is_super_admin))
            .bind(("created_by", storable_admin_user.logged_in_user.clone()))
            .bind(("updated_by", storable_admin_user.logged_in_user))
            .bind(("created_at", Datetime::default()))
            .bind(("updated_at", Datetime::default()))
            .await?;

        println!("create_admin_user_response: {:#?}", create_admin_user_response);
        let results = create_admin_user_response.take::<Value>(0).ok();

        
        // return false
        let user = match results {
            Some(value) => value,
            None => return Err(crate::error::Error::Generic("Failed to create admin user".to_string())),
        };
        
        println!("Results: {:#?}", user);

        // let user = user.try_into().unwrap();
            // @todo create an admin user


        Ok(true)
    }
}
