use crate::infra::app::DB;

pub struct SurrealUserRepository {
    pub db: DB,
}

impl SurrealUserRepository {
    pub fn new(db: DB) -> Self {
        Self { db }
    }
}



// #[derive(Clone)]
// pub struct DatabaseRepository {
//     pub pool: PgPool,
// }
