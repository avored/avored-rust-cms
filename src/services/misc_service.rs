use crate::api::proto::misc::{SetupRequest, SetupResponse};
use crate::error::Result;
use crate::providers::avored_database_provider::DB;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
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
        let sql = "

        REMOVE TABLE settings;
        DEFINE TABLE settings;

        DEFINE FIELD identifier ON TABLE settings TYPE string;
        DEFINE FIELD value      ON TABLE settings TYPE string;
        DEFINE FIELD created_at ON TABLE settings TYPE datetime;
        DEFINE FIELD updated_at ON TABLE settings TYPE datetime;
        DEFINE FIELD created_by ON TABLE settings TYPE string;
        DEFINE FIELD updated_by ON TABLE settings TYPE string;

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


        REMOVE TABLE admin_users;
        DEFINE TABLE admin_users;

        DEFINE FIELD full_name ON TABLE admin_users TYPE string;
        DEFINE FIELD email ON TABLE admin_users TYPE string;
        DEFINE FIELD password ON TABLE admin_users TYPE string;
        DEFINE FIELD profile_image ON TABLE admin_users TYPE string;
        DEFINE FIELD is_super_admin ON TABLE admin_users TYPE bool;
        DEFINE FIELD created_by ON TABLE admin_users TYPE string;
        DEFINE FIELD updated_by ON TABLE admin_users TYPE string;
        DEFINE FIELD created_at ON TABLE admin_users TYPE datetime;
        DEFINE FIELD updated_at ON TABLE admin_users TYPE datetime;
        DEFINE INDEX admin_users_email_index ON TABLE admin_users COLUMNS email UNIQUE;

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

        REMOVE TABLE password_rest;
        DEFINE TABLE password_rest;

        DEFINE FIELD email ON TABLE password_rest TYPE string;
        DEFINE FIELD token ON TABLE password_rest TYPE string;
        DEFINE FIELD created_at ON TABLE password_rest TYPE datetime;


        REMOVE TABLE roles;
        DEFINE TABLE roles;

        DEFINE FIELD name ON TABLE roles TYPE string;
        DEFINE FIELD identifier ON TABLE roles TYPE string;
        DEFINE FIELD created_by ON TABLE roles TYPE string;
        DEFINE FIELD updated_by ON TABLE roles TYPE string;
        DEFINE FIELD created_at ON TABLE roles TYPE datetime;
        DEFINE FIELD updated_at ON TABLE roles TYPE datetime;
        DEFINE INDEX roles_identifier_index ON TABLE roles COLUMNS identifier UNIQUE;

        CREATE roles CONTENT {
            name: 'Administrator',
            identifier: 'administrator',
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };


        REMOVE TABLE admin_user_role;
        DEFINE TABLE admin_user_role;

        DEFINE FIELD created_by ON TABLE admin_user_role TYPE string;
        DEFINE FIELD updated_by ON TABLE admin_user_role TYPE string;
        DEFINE FIELD created_at ON TABLE admin_user_role TYPE datetime;
        DEFINE FIELD updated_at ON TABLE admin_user_role TYPE datetime;


        REMOVE TABLE components;
        DEFINE TABLE components;

        DEFINE FIELD name ON TABLE components TYPE string;
        DEFINE FIELD identifier ON TABLE components TYPE string;
        DEFINE FIELD created_by ON TABLE components TYPE string;
        DEFINE FIELD updated_by ON TABLE components TYPE string;
        DEFINE FIELD created_at ON TABLE components TYPE datetime;
        DEFINE FIELD updated_at ON TABLE components TYPE datetime;
        DEFINE INDEX components_identifier_index ON TABLE components COLUMNS identifier UNIQUE;


        REMOVE TABLE pages;
        DEFINE TABLE pages;

        DEFINE FIELD name ON TABLE pages TYPE string;
        DEFINE FIELD identifier ON TABLE pages TYPE string;
        DEFINE FIELD created_by ON TABLE pages TYPE string;
        DEFINE FIELD updated_by ON TABLE pages TYPE string;
        DEFINE FIELD created_at ON TABLE pages TYPE datetime;
        DEFINE FIELD updated_at ON TABLE pages TYPE datetime;
        DEFINE INDEX pages_identifier_index ON TABLE pages COLUMNS identifier UNIQUE;

        REMOVE TABLE assets;
        DEFINE TABLE assets;


        DEFINE TABLE fields;

        REMOVE TABLE collections;
        DEFINE TABLE collections;

        DEFINE FIELD name ON TABLE collections TYPE string;
        DEFINE FIELD identifier ON TABLE collections TYPE string;
        DEFINE FIELD created_by ON TABLE collections TYPE string;
        DEFINE FIELD updated_by ON TABLE collections TYPE string;
        DEFINE FIELD created_at ON TABLE collections TYPE datetime;
        DEFINE FIELD updated_at ON TABLE collections TYPE datetime;
        DEFINE INDEX collections_identifier_index ON TABLE collections COLUMNS identifier UNIQUE;
        DEFINE INDEX collections_identifier_index ON TABLE collections COLUMNS identifier UNIQUE;

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
    pub async fn new() -> Result<MiscService> {
        Ok(MiscService {})
    }
}
