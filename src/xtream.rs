use serde::de::DeserializeOwned;
use serde::Deserialize;
/// Typage et structure
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub user_info: UserInfo,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub auth: u8,
    pub status: String,
}
#[derive(Debug, Deserialize)]
pub struct Category {
    pub category_id: String,
    pub category_name: String,
    pub parent_id: u8,
}

#[derive(Debug, Deserialize)]
pub struct LiveContent {
    pub num: u32,
    pub name: String,
    pub stream_type: String,
    pub stream_id: u32,
    pub stream_icon: String,
    pub category_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct SeriesContent {
    pub num: u32,
    pub name: String,
    pub series_id: u32,
    pub cover: String,
    pub plot: String,
    pub cast: String,
    pub director: String,
    pub genre: String,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    pub last_modified: String,
    pub rating: String,
    pub youtube_trailer: String,
    pub episode_run_time: String,
    pub category_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct VodContent {
    pub num: u32,
    pub name: String,
    pub stream_type: String,
    pub stream_id: u32,
    pub stream_icon: Option<String>,
    pub rating_5based: f32,
    pub container_extension: String,
    pub category_id: Option<String>,
}

/// Api call
pub fn api<T: DeserializeOwned>(
    host: &str,
    username: &str,
    password: &str,
    action: Option<&str>,
) -> T {
    let url = build_api_url(host, username, password, action);
    let client = reqwest::blocking::Client::builder()
        .user_agent("VLC/3.0.20 LibVLC/3.0.20")
        .build()
        .expect("User Agent error");
    let resp = client.get(&url).send().expect("Send error");
    let status = resp.status();
    if !status.is_success() {
        panic!("Requête refusé (Status : {status})")
    }
    let body = resp.text().expect("body text absent");
    let parsed: T = serde_json::from_str(&body).expect("Parsing error");
    parsed
}

fn build_api_url(host: &str, username: &str, password: &str, action: Option<&str>) -> String {
    let mut url = format!("{host}/player_api.php?username={username}&password={password}");

    if let Some(a) = action {
        url.push_str(&format!("&action={a}"));
    }
    url
}
