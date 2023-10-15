use serde::{Deserialize, Serialize};

use super::tab_content::Content;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub content: Content,
}

impl Tab {
    pub fn new(id: usize, title: String, content: Content) -> Tab {
        Tab { id, title, content }
    }
}
