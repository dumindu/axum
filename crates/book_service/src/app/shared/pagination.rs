use serde::Deserialize;

const DEFAULT_PER_PAGE: usize = 10;
const MAX_PER_PAGE: usize = 100;

#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_per_page")]
    pub per_page: usize,
    #[serde(default = "default_page")]
    pub page: usize,
}

fn default_per_page() -> usize {
    DEFAULT_PER_PAGE
}
fn default_page() -> usize {
    1
}

impl Pagination {
    pub fn limit_offset(self) -> (usize, usize) {
        let limit = self.per_page.clamp(1, MAX_PER_PAGE);
        let page = std::cmp::max(1, self.page);
        let offset = (page - 1) * limit;
        (limit, offset)
    }
}
