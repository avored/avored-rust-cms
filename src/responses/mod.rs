use serde::Serialize;

pub mod role;
pub mod page;
pub mod component;
pub mod model;

#[derive(Serialize)]
pub struct ApiResponse<R> {
    pub status: bool,
    pub data: R
}