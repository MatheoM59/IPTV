use serde::Deserialize;
use serde::de::DeserializeOwned;

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
pub enum StreamKind {
    Live,
    Movie,
    Series,
}

/// Api call
pub fn api<T: DeserializeOwned>(
    host: &str,
    username: &str,
    password: &str,
    action: Option<&str>,
) -> Result<T, String> {
    let url = build_api_url(host, username, password, action);
    let client = reqwest::blocking::Client::builder()
        .user_agent("VLC/3.0.20 LibVLC/3.0.20")
        .build()
        .map_err(|e| format!("User Agent error {e}"))?;
    let resp = client
        .get(&url)
        .send()
        .map_err(|e| format!("Send error {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(format!("Requête refusé (status : {status})"));
    }
    let body = resp.text().map_err(|e| format!("Body text absent {e}"))?;
    serde_json::from_str(&body).map_err(|e| format!("Parse error {e}"))
}

fn build_api_url(host: &str, username: &str, password: &str, action: Option<&str>) -> String {
    let mut url = format!("{host}/player_api.php?username={username}&password={password}");

    if let Some(a) = action {
        url.push_str(&format!("&action={a}"));
    }
    url
}

fn build_stream_url(
    host: &str,
    username: &str,
    password: &str,
    kind: StreamKind,
    stream_id: u32,
    extension: &str,
) -> String {
    let segment = match kind {
        StreamKind::Live => "live",
        StreamKind::Movie => "movie",
        StreamKind::Series => "series",
    };
    format!("{host}/{segment}/{username}/{password}/{stream_id}.{extension}")
}
