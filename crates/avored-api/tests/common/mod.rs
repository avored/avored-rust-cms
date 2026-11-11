use avored_api::{
    core::domain::extensions::string_extension::StringExtension,
    providers::avored_database_provider::AvoRedDatabaseProvider,
};

pub async fn seed_test_user(
    provider: &AvoRedDatabaseProvider,
) {
    let (datastore, session) = &provider.db;
    let password = "secret"
        .to_string()
        .get_password_hash("test-salt")
        .expect("test password should hash");
    let sql = format!(
        "CREATE users:test_user SET name = 'Test User', email = 'test@example.com', password = '{}', created_at = time::now(), created_by = 'test', updated_at = time::now(), updated_by = 'test', deleted_at = NONE RETURN AFTER;",
        password
    );

    datastore
        .execute(&sql, session, None)
        .await
        .expect("test user should be created");
}
