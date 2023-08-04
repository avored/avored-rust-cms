#[derive(Debug, Clone)]
pub struct Config {
    pub database_namespace: String,
    pub database_name: String,
}

impl Config {
    pub fn new() -> Config {
        let database_namespace = std::env::var("AVORED_DATABASE_NAME").expect("AVORED_DATABASE_NAMESPACE must be set");
        let database_name = std::env::var("AVORED_DATABASE_NAME").expect("AVORED_DATABASE_NAME must be set");

        Config {
            database_namespace,
            database_name,
        }
    }
}
