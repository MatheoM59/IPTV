use crate::xtream::{Content, Credentials, UserInfo};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, SystemTime},
};
#[derive(Debug, Deserialize, Serialize)]
pub struct AppState {
    pub session: Mutex<Option<Session>>,
    pub catalog: Mutex<HashMap<String, CachedCatalog>>,
}

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
pub struct Session {
    pub credentials: Credentials,
    pub user_info: UserInfo,
}

#[derive(Debug, Serialize, Clone)]
pub struct AccountView {
    pub host: String,
    pub username: String,
    pub status: String,
    pub exp_date: Option<String>,
}
