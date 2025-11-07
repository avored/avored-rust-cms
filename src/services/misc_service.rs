use crate::api::proto::misc::{SetupRequest, SetupResponse};
use crate::error::Result;
use crate::providers::avored_database_provider::DB;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use tokio::fs;
use std::collections::BTreeMap;
/// misc service
pub struct MiscService {}

impl MiscService {
    pub(crate) async fn setup(
        &self,
        req: SetupRequest,
        password_salt: &str,
        (ds, ses): &DB,
    ) -> Result<SetupResponse> {

        // let (ds, ses) = &self.state.db;
        let mut entries = fs::read_dir("./migrations").await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                println!("Reading file: {path:?}");
                let sql = fs::read_to_string(&path).await?;

                let responses = ds.execute(&sql, ses, None).await.unwrap();

                println!("Content of {path:?}:\n{responses:?}");
            } else if path.is_dir() {
                println!("Found directory: {path:?}");
                // You could recursively call a function here to read subdirectories
            }
        }

        let sql = "
            CREATE settings CONTENT {
                identifier: 'general_site_name',
                value: 'Avored rust cms',
                created_by: $email,
                updated_by: $email,
                created_at: time::now(),
                updated_at: time::now()
            };

            CREATE settings CONTENT {
                identifier: 'auth_cms_token',
                value: '',
                created_by: $email,
                updated_by: $email,
                created_at: time::now(),
                updated_at: time::now()
            };


        
            CREATE admin_users CONTENT {
                full_name: $full_name,
                email: $email,
                password: $password,
                profile_image: $profile_image,
                is_super_admin: $is_super_admin,
                created_by: $email,
                updated_by: $email,
                created_at: time::now(),
                updated_at: time::now()
            };

            CREATE roles CONTENT {
                name: 'Administrator',
                identifier: 'administrator',
                created_by: $email,
                updated_by: $email,
                created_at: time::now(),
                updated_at: time::now()
            };



            CREATE collections CONTENT {
                name: 'Pages',
                identifier: 'pages',
                created_by: $email,
                updated_by: $email,
                created_at: time::now(),
                updated_at: time::now()
            };
        ";

        let password = req.password.as_bytes();
        let salt = SaltString::from_b64(password_salt).unwrap();

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password, &salt)
            .expect("Error occurred while encrypted password")
            .to_string();

        let vars = BTreeMap::from([
            ("full_name".into(), "Admin".into()),
            ("email".into(), req.email.into()),
            ("password".into(), password_hash.as_str().into()),
            ("profile_image".into(), "".into()),
            ("is_super_admin".into(), true.into()),
        ]);

        let responses = ds.execute(sql, ses, Some(vars)).await.unwrap();

        println!("{responses:?}");
        println!();
        println!("Migrate fresh done!");

        let reply = SetupResponse { status: true };

        Ok(reply)
    }
}


impl MiscService {
    /// create misc service instance
    pub async fn new() -> Result<Self> {
        Ok(Self {})
    }
}
