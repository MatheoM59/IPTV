use crate::xtream::{Content, Credentials};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, SystemTime},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CachedCatalog {
    pub items: Vec<Content>,
    pub fetched_at: SystemTime,
}

impl CachedCatalog {
    pub fn is_fresh(&self) -> bool {
        let ttl = Duration::from_secs(24 * 60 * 60);
        self.fetched_at.elapsed().map(|e| e < ttl).unwrap_or(false)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppState {
    pub credentials: Mutex<Option<Credentials>>,
    pub catalog: Mutex<HashMap<String, CachedCatalog>>,
}
