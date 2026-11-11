use serde::{Deserialize, Serialize};

pub const DEFAULT_PER_PAGE: u64 = 25;
pub const MAX_PER_PAGE: u64 = 100;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pagination {
    pub total: u64,
    pub per_page: u64,
    pub current_page: u64,
    pub last_page: u64,
}

impl Pagination {
    pub fn new(total: u64, per_page: u64, current_page: u64) -> Self {
        let per_page = per_page.clamp(1, MAX_PER_PAGE);
        Self {
            total,
            per_page,
            current_page: current_page.max(1),
            last_page: if total == 0 {
                1
            } else {
                total.div_ceil(per_page)
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub page_size: Option<u64>,
    pub limit: Option<u64>,
}

impl PaginationQuery {
    pub fn current_page(&self) -> u64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> u64 {
        self.per_page
            .or(self.page_size)
            .or(self.limit)
            .unwrap_or(DEFAULT_PER_PAGE)
            .clamp(1, MAX_PER_PAGE)
    }
}

#[cfg(test)]
mod tests {
    use super::{PaginatedResponse, Pagination, PaginationQuery, DEFAULT_PER_PAGE};

    #[test]
    fn query_defaults_and_clamps_pagination() {
        let defaults = PaginationQuery::default();
        assert_eq!(defaults.current_page(), 1);
        assert_eq!(defaults.per_page(), DEFAULT_PER_PAGE);

        let clamped = PaginationQuery {
            page: Some(0),
            per_page: Some(500),
            ..PaginationQuery::default()
        };
        assert_eq!(clamped.current_page(), 1);
        assert_eq!(clamped.per_page(), 100);
    }

    #[test]
    fn query_supports_legacy_page_size_aliases() {
        let page_size = PaginationQuery {
            page_size: Some(10),
            ..PaginationQuery::default()
        };
        let limit = PaginationQuery {
            limit: Some(5),
            ..PaginationQuery::default()
        };
        assert_eq!(page_size.per_page(), 10);
        assert_eq!(limit.per_page(), 5);
    }

    #[test]
    fn empty_results_have_one_last_page() {
        let response: PaginatedResponse<()> = PaginatedResponse {
            data: Vec::new(),
            pagination: Pagination::new(0, 25, 1),
        };
        assert_eq!(response.pagination.last_page, 1);
    }
}
