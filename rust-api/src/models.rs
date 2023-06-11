use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize};
use uuid::Uuid;
use crate::schema::posts;

#[derive(Queryable, Serialize, Debug)]
#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime
}
