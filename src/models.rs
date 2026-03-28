use serde::Deserialize;

#[derive(Deserialize)]
pub struct Link {
    pub id: String,
    pub title: Option<String>,
}

#[derive(Deserialize)]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}
