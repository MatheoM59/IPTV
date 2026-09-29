use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Typage et structure
#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    pub user_info: UserInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub auth: u8,
    pub status: String,
    pub exp_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Credentials {
    pub host: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub category_id: String,
    pub category_name: String,
    pub parent_id: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiveContent {
    pub num: u32,
    pub name: String,
    pub stream_type: String,
    pub stream_id: u32,
    pub stream_icon: String,
    pub category_id: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Content {
    pub id: u32,
    pub title: String,
    pub image: Option<String>,
    pub category_id: Option<String>,
    pub extention: Option<String>,
}

impl From<LiveContent> for Content {
    fn from(c: LiveContent) -> Self {
        Content {
            id: c.stream_id,
            title: c.name,
            image: Some(c.stream_icon),
            category_id: c.category_id,
            extention: None,
        }
    }
}
impl From<VodContent> for Content {
    fn from(c: VodContent) -> Self {
        Content {
            id: c.stream_id,
            title: c.name,
            image: c.stream_icon,
            category_id: c.category_id,
            extention: Some(c.container_extension),
        }
    }
}

impl From<SeriesContent> for Content {
    fn from(c: SeriesContent) -> Self {
        Content {
            id: c.series_id,
            title: c.name,
            image: Some(c.cover),
            category_id: c.category_id,
            extention: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Rating {
    Num(f32),
    Text(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum GenreField {
    One(String),
    Many(Vec<String>),
}

fn non_vide(s: String) -> Option<String> {
    if s.trim().is_empty() { None } else { Some(s) }
}

/// Le dict ffprobe `info.video` compte 29 cles ; serde ignore celles
/// qu'on ne declare pas. Seule la largeur nous interesse.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VideoStream {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Classer sur la LARGEUR, jamais sur la hauteur : le format cinema
/// recadre l'image (1920x804, 1920x1072…) et un test sur la hauteur
/// declasserait des films en pleine resolution.
/// Des seuils, pas des egalites : un film mesure sort a 1904, pas 1920.
fn qualite(largeur: Option<u32>) -> Option<String> {
    let w = largeur?;
    Some(
        match w {
            0 => return None,
            w if w >= 3840 => "4K",
            w if w >= 1900 => "1080p",
            w if w >= 1280 => "720p",
            _ => "SD",
        }
        .to_string(),
    )
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(default)]
pub struct MovieInfo {
    pub name: String,
    pub plot: String,
    pub cast: String,
    pub director: String,
    pub genre: Option<GenreField>,
    pub duration: String,
    pub releasedate: Option<String>,
    pub rating: Option<Rating>,
    pub age: String,
    pub country: String,
    pub backdrop_path: Option<Vec<String>>,
    pub youtube_trailer: String,
    // Deux replis pour le fond du bandeau quand `backdrop_path` manque.
    pub cover_big: String,
    pub movie_image: Option<String>,
    // Metadonnees ffprobe : seule la resolution nous sert.
    pub video: Option<VideoStream>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(default)]
pub struct MovieData {
    pub stream_id: u32,
    pub container_extension: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MovieResponse {
    pub info: MovieInfo,
    pub movie_data: MovieData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MovieDetails {
    pub name: String,
    pub duration: String,
    pub stream_id: u32,
    pub container_extension: String,

    pub plot: Option<String>,
    pub cast: Option<String>,
    pub director: Option<String>,
    pub genre: Option<String>,
    pub releasedate: Option<String>,
    pub rating: Option<f32>,
    pub age: Option<String>,
    pub country: Option<String>,
    pub backdrop_path: Option<String>,
    pub youtube_trailer: Option<String>,
    /// « 4K », « 1080p », « 720p » ou « SD ». Deduit de la largeur reelle
    /// du fichier, pas d'une promesse du panel.
    pub quality: Option<String>,
}
impl From<MovieResponse> for MovieDetails {
    fn from(r: MovieResponse) -> Self {
        let info = r.info;
        let data = r.movie_data;
        let backdrop = info
            .backdrop_path
            .unwrap_or_default()
            .into_iter()
            .find(|u| !u.trim().is_empty())
            .or_else(|| non_vide(info.cover_big))
            .or_else(|| info.movie_image.and_then(non_vide));

        MovieDetails {
            name: non_vide(info.name).unwrap_or(data.name),
            duration: info.duration,
            stream_id: data.stream_id,
            container_extension: data.container_extension,
            plot: non_vide(info.plot),
            cast: non_vide(info.cast),
            director: non_vide(info.director),
            genre: match info.genre {
                Some(GenreField::One(s)) => non_vide(s),
                Some(GenreField::Many(v)) => non_vide(v.join(" / ")),
                None => None,
            },
            releasedate: info.releasedate.and_then(non_vide),
            rating: match info.rating {
                Some(Rating::Num(n)) => Some(n),
                Some(Rating::Text(s)) => s.trim().parse().ok(),
                None => None,
            },
            age: non_vide(info.age),
            country: non_vide(info.country),
            backdrop_path: backdrop,
            youtube_trailer: non_vide(info.youtube_trailer),
            quality: qualite(info.video.and_then(|v| v.width)),
        }
    }
}

/// Api call
pub async fn api<T: DeserializeOwned>(
    creds: &Credentials,
    action: Option<&str>,
    params: &[(&str, &str)],
) -> Result<T, String> {
    let url = build_api_url(creds, action, params);
    let client = reqwest::Client::builder()
        .user_agent("VLC/3.0.20 LibVLC/3.0.20")
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("User Agent error {e}"))?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Send error {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        return Err(format!("Requête refusé (status : {status})"));
    }
    let body = resp
        .text()
        .await
        .map_err(|e| format!("Body text absent {e}"))?;

    serde_json::from_str(&body).map_err(|e| format!("Parse error {e}"))
}

fn build_api_url(creds: &Credentials, action: Option<&str>, params: &[(&str, &str)]) -> String {
    let Credentials {
        host,
        username,
        password,
    } = creds;

    let mut url = format!("{host}/player_api.php?username={username}&password={password}");

    if let Some(a) = action {
        url.push_str(&format!("&action={a}"));
    }
    for (k, v) in params {
        url.push_str(&format!("&{k}={v}"))
    }
    url
}

pub fn build_stream_url(
    host: &str,
    kind: StreamKind,
    username: &str,
    password: &str,
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
