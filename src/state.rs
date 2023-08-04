use surrealdb::{dbs::Session, kvs::Datastore};

use crate::config::Config;

pub struct State {
    pub datastore: Datastore,
    pub database_session: Session
}

impl State {
    pub async fn new(config: Config) -> State {

        let datastore = Datastore::new("file://data/avored.db")
            .await
            .expect("there is issue with connecting with data/avored.db storage");

        let database_session = Session::for_db(
            config.database_namespace.clone(),
            config.database_name.clone(),
        );
        
        State {
            datastore,
            database_session,
        }
    }
}
