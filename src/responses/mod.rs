use serde::Serialize;

pub mod component;
pub mod model;
pub mod page;
pub mod role;

#[derive(Serialize)]
pub struct ApiResponse<R> {
    pub status: bool,
    pub data: R,
}
