use serde::{Deserialize, Serialize};

use crate::core::domain::entities::EmailTemplateModel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginateEmailTemplateCommand {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailTemplatePaginationResponse {
    pub data: Vec<EmailTemplateModel>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
