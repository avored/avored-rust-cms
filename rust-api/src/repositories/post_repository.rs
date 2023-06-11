use diesel::{PgConnection};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::{prelude::*};

use crate::{ models::Post};
use crate::schema::posts::dsl::*;

pub struct PostRepository {
    pub db: Pool<ConnectionManager<PgConnection>>,
}

impl PostRepository {
    pub fn new(db: Pool<ConnectionManager<PgConnection>> ) -> PostRepository {
        PostRepository { db }
    }

    pub fn all(&self) -> Vec<Post> {
        let conn = &mut self.db.get().unwrap();

        posts
            .load(conn)
            .expect("Error loading posts")
    }
}
