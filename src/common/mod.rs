use serde::{Deserialize, Serialize};

/// 通用分页入参：配合 axum `Query` 提取器使用，各列表接口通过 `#[serde(flatten)]` 复用
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    10
}

impl PaginationQuery {
    /// 页码，最小为 1
    pub fn page(&self) -> u32 {
        self.page.max(1)
    }

    /// 每页条数，钳制在 1..=100
    pub fn per_page(&self) -> u32 {
        self.per_page.clamp(1, 100)
    }

    pub fn offset(&self) -> i64 {
        ((self.page() - 1) * self.per_page()) as i64
    }

    pub fn limit(&self) -> i64 {
        self.per_page() as i64
    }
}

/// 统一分页响应包装，所有列表接口统一返回该结构
#[derive(Debug, Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
}

impl<T> Paginated<T> {
    pub fn new(items: Vec<T>, page: u32, per_page: u32, total: u64) -> Self {
        let total_pages = if per_page == 0 {
            0
        } else {
            total.div_ceil(per_page as u64) as u32
        };
        Self {
            items,
            page,
            per_page,
            total,
            total_pages,
        }
    }
}
