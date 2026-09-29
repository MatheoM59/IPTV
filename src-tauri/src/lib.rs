use crate::app::{AccountView, AppState, CachedCatalog, Session};
use crate::xtream::{
    ApiResponse, Category, Content, Credentials, LiveContent, MovieDetails, MovieResponse,
    SeriesContent, StreamKind, VodContent,
};
use std::time::SystemTime;
use std::{collections::HashMap, sync::Mutex};
mod app;
mod gemini;
mod xtream;

#[tauri::command]
async fn get_account_info(
    state: tauri::State<'_, AppState>,
    host: String,
    username: String,
    password: String,
) -> Result<ApiResponse, String> {
    let creds = Credentials {
        host,
        username,
        password,
    };
    let account = xtream::api::<ApiResponse>(&creds, None, &[]).await?;
    if account.user_info.auth != 1 {
        return Err("Erreur de connection".to_string());
    }
    if !account.user_info.status.eq_ignore_ascii_case("active") {
        return Err(format!(
            "Abonnement non actif : {}",
            account.user_info.status
        ));
    }
    let session = Session {
        credentials: creds,
        user_info: account.user_info.clone(),
    };
    let mut guard = state.session.lock().map_err(
        |e: std::sync::PoisonError<std::sync::MutexGuard<'_, Option<Session>>>| {
            format!("État vérouillé : {e}")
        },
    )?;
    *guard = Some(session);
    Ok(account)
}

#[tauri::command]
async fn get_categories(
    state: tauri::State<'_, AppState>,
    catalog: String,
) -> Result<Vec<Category>, String> {
    let creds = {
        let guard = state
            .session
            .lock()
            .map_err(|e| format!("État vérouillé : {e}"))?;
        guard
            .as_ref()
            .ok_or("Aucune connection active")?
            .credentials
            .clone()
    };
    let action = match catalog.as_str() {
        "live" => "get_live_categories",
        "vod" => "get_vod_categories",
        "serie" => "get_series_categories",
        other => return Err(format!("Catalogue inconnue : {other}")),
    };
    xtream::api::<Vec<Category>>(&creds, Some(action), &[]).await
}

#[tauri::command]
fn get_status(state: tauri::State<AppState>) -> Result<bool, String> {
    let guard = state
        .session
        .lock()
        .map_err(|e| format!("État vérouillé : {e}"))?;
    Ok(guard.is_some())
}

#[tauri::command]
fn get_account(state: tauri::State<AppState>) -> Result<AccountView, String> {
    let session = state
        .session
        .lock()
        .map_err(|e| format!("Vérouillé : {e}"))?;
    let unlock = session.as_ref().ok_or("Pas de session")?;

    let host = unlock.credentials.host.clone();
    let username = unlock.credentials.username.clone();
    let status = unlock.user_info.status.clone();
    let exp_date = unlock.user_info.exp_date.clone();
    let result = AccountView {
        host,
        username,
        status,
        exp_date,
    };
    Ok(result)
}

async fn ensure_catalog(state: &AppState, catalog: &str) -> Result<(), String> {
    {
        let cache = state
            .catalog
            .lock()
            .map_err(|e| format!("Cache vérouillé : {e}"))?;
        if let Some(entry) = cache.get(catalog)
            && entry.is_fresh()
        {
            return Ok(());
        }
    };

    let creds = {
        let guard = state
            .session
            .lock()
            .map_err(|e| format!("État vérouillé : {e}"))?;
        guard
            .as_ref()
            .ok_or("Aucune connection acitve")?
            .credentials
            .clone()
    };
    let items: Vec<Content> = match catalog {
        "live" => xtream::api::<Vec<LiveContent>>(&creds, Some("get_live_streams"), &[])
            .await?
            .into_iter()
            .map(Content::from)
            .collect(),
        "vod" => xtream::api::<Vec<VodContent>>(&creds, Some("get_vod_streams"), &[])
            .await?
            .into_iter()
            .map(Content::from)
            .collect(),
        "serie" => xtream::api::<Vec<SeriesContent>>(&creds, Some("get_series"), &[])
            .await?
            .into_iter()
            .map(Content::from)
            .collect(),
        other => return Err(format!("Catalogue inconnue : {other}")),
    };
    {
        let mut cache = state
            .catalog
            .lock()
            .map_err(|e| format!("État vérouillé : {e}"))?;
        cache.insert(
            catalog.to_string(),
            CachedCatalog {
                items,
                fetched_at: SystemTime::now(),
            },
        )
    };
    Ok(())
}

#[tauri::command]
async fn get_content(
    state: tauri::State<'_, AppState>,
    catalog: String,
    category_id: Option<String>,
) -> Result<Vec<Content>, String> {
    ensure_catalog(&state, &catalog).await?;
    let cache = state
        .catalog
        .lock()
        .map_err(|e| format!("État vérouillé : {e}"))?;
    let entry = cache
        .get(&catalog)
        .ok_or("Catalog absent du cache lors du chargement")?;

    let result = filter_cate(&entry.items, category_id.as_deref());
    Ok(result)
}

#[tauri::command]
async fn get_vod_details(
    state: tauri::State<'_, AppState>,
    vod_id: String,
) -> Result<MovieDetails, String> {
    let creds = {
        let guard = state
            .session
            .lock()
            .map_err(|e| format!("État vérouillé : {e}"))?;
        guard
            .as_ref()
            .ok_or("Aucune connection active")?
            .credentials
            .clone()
    };
    let action = "get_vod_info";
    Ok(
        xtream::api::<MovieResponse>(&creds, Some(action), &[("vod_id", &vod_id)])
            .await?
            .into(),
    )
}

#[tauri::command]
async fn search_content(
    state: tauri::State<'_, AppState>,
    catalog: String,
    query: String,
) -> Result<Vec<Content>, String> {
    ensure_catalog(&state, &catalog).await?;
    let cache = state
        .catalog
        .lock()
        .map_err(|e| format!("État vérouillé : {e}"))?;
    let entry = cache
        .get(&catalog)
        .ok_or("Catalog absent du cache lors du chargement")?;

    let result = filter_title(&entry.items, &query);
    Ok(result)
}

fn filter_cate(items: &[Content], category_id: Option<&str>) -> Vec<Content> {
    match category_id {
        None => items.to_vec(),
        Some(id) => items
            .iter()
            .filter(|c| c.category_id.as_deref() == Some(id))
            .cloned()
            .collect(),
    }
}

fn filter_title(items: &[Content], query: &str) -> Vec<Content> {
    let query_low = query.to_lowercase();
    items
        .iter()
        .filter(|c| c.title.to_lowercase().contains(&query_low))
        .take(200)
        .cloned()
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            session: Mutex::new(None),
            catalog: Mutex::new(HashMap::new()),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_account_info,
            get_categories,
            get_status,
            get_content,
            search_content,
            get_account,
            get_vod_details,
            lire
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
/* #[tauri::command]
pub fn geminiCall() {
    let result = gemini::chercher(&requete, nb).await?;
    ensure_catalog(state, "vod");
    ensure_catalog(state, "serie");
    filter_title(items, query)
}
 */

#[tauri::command]
async fn lire(
    state: tauri::State<'_, AppState>,
    catalog: String,
    id: u32,
    extension: Option<String>,
    titre: String,
) -> Result<(), String> {
    let creds = {
        let guard = state
            .session
            .lock()
            .map_err(|e| format!("État vérouillé : {e}"))?;
        guard
            .as_ref()
            .ok_or("Aucune connection active")?
            .credentials
            .clone()
    };
    let (kind, ext) = match catalog.as_str() {
        "live" => (StreamKind::Live, "ts".to_string()),
        "vod" => (StreamKind::Movie, extension.ok_or("Extension manquante")?),
        "serie" => (StreamKind::Series, extension.ok_or("Extension manquante")?),
        other => return Err(format!("Catalogue inconnue : {other}")),
    };
    let url = xtream::build_stream_url(
        &creds.host,
        kind,
        &creds.username,
        &creds.password,
        id,
        &ext,
    );
    println!("URL : {}", url.replace(&creds.password, "***"));

    let mut cmd = std::process::Command::new("open");
    cmd.arg("-a").arg("IINA").arg(&url).arg("--mpv-fullscreen");

    if catalog == "live" {
        cmd.arg("--mpv-no-resume-playback");
    }
    cmd.spawn()
        .map_err(|e| format!("Lecteur IINA introuvable : {e}"))?;
    Ok(())
}
