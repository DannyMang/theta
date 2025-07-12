use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub favicon: Option<String>,
    pub is_loading: bool,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub history: Vec<String>,
}

pub struct TabManager {
    tabs: HashMap<Uuid, Tab>,
    active_tab: Option<Uuid>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            active_tab: None,
        }
    }

    pub fn create_tab(&mut self, url: String, title: String) -> Uuid {
        let id = Uuid::new_v4();
        let tab = Tab {
            id,
            title,
            url: url.clone(),
            favicon: None,
            is_loading: false,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            history: vec![url],
        };
        
        self.tabs.insert(id, tab);
        self.active_tab = Some(id);
        id
    }

    pub fn get_tab(&self, id: &Uuid) -> Option<&Tab> {
        self.tabs.get(id)
    }

    pub fn get_tab_mut(&mut self, id: &Uuid) -> Option<&mut Tab> {
        self.tabs.get_mut(id)
    }

    pub fn close_tab(&mut self, id: &Uuid) -> bool {
        if self.tabs.remove(id).is_some() {
            if self.active_tab == Some(*id) {
                self.active_tab = self.tabs.keys().next().cloned();
            }
            true
        } else {
            false
        }
    }

    pub fn set_active_tab(&mut self, id: Uuid) -> bool {
        if self.tabs.contains_key(&id) {
            self.active_tab = Some(id);
            if let Some(tab) = self.tabs.get_mut(&id) {
                tab.last_accessed = Utc::now();
            }
            true
        } else {
            false
        }
    }

    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.active_tab.and_then(|id| self.tabs.get(&id))
    }

    pub fn get_all_tabs(&self) -> Vec<&Tab> {
        self.tabs.values().collect()
    }

    pub fn navigate_tab(&mut self, id: &Uuid, url: String) -> bool {
        if let Some(tab) = self.tabs.get_mut(id) {
            tab.url = url.clone();
            tab.history.push(url);
            tab.last_accessed = Utc::now();
            tab.is_loading = true;
            true
        } else {
            false
        }
    }

    pub fn update_tab_title(&mut self, id: &Uuid, title: String) -> bool {
        if let Some(tab) = self.tabs.get_mut(id) {
            tab.title = title;
            true
        } else {
            false
        }
    }

    pub fn set_tab_loading(&mut self, id: &Uuid, is_loading: bool) -> bool {
        if let Some(tab) = self.tabs.get_mut(id) {
            tab.is_loading = is_loading;
            true
        } else {
            false
        }
    }
} 