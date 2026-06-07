use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct PageQuery {
    pub page: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
}

impl PageQuery {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(10).clamp(1, 100)
    }

    pub fn offset(&self) -> u64 {
        ((self.page() - 1) * self.page_size()) as u64
    }
}

#[derive(Debug, Serialize)]
pub struct PageResult<T>
where
    T: Serialize,
{
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    #[serde(rename = "pageSize")]
    pub page_size: u32,
}
