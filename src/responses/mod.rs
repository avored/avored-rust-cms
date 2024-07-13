use serde::Serialize;

pub mod role;
pub mod page;

#[derive(Serialize)]
pub struct ApiResponse<R> {
    pub status: bool,
    pub data: R
}