use super::{tab::Tab, tab_content::Content};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manager {
    pub tabs: Vec<Tab>,
    pub tab_amount: usize,
}

impl Manager {
    pub fn new() -> Manager {
        let tabs = Vec::new();
        Manager {
            tabs,
            tab_amount: 0,
        }
    }

    pub fn create_tab(&mut self, title: String, content: Content) -> usize {
        self.tab_amount += 1;
        self.tabs.push(Tab::new(self.tab_amount, title, content));
        self.tab_amount
    }

    pub fn close_tab_by_id(&mut self, id: usize) {
        let pos = self.tabs.iter().position(|v| v.id == id).unwrap();
        self.tabs.remove(pos);
    }

    pub fn move_tab(&mut self, from: usize, to: usize) {
        self.tabs.swap(from, to)
    }
}
