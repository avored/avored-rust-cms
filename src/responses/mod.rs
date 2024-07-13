use serde::Serialize;

pub mod role;

#[derive(Serialize)]
pub struct ApiResponse<R> {
    pub status: bool,
    pub data: R
}