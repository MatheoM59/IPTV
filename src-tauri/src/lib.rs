use crate::app::{AccountView, AppState, CachedCatalog, Session};
use crate::xtream::{
    ApiResponse, Category, Content, Credentials, LiveContent, SeriesContent, VodContent,
};
use std::time::SystemTime;
use std::{collections::HashMap, sync::Mutex};
mod app;
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
    let account = xtream::api::<ApiResponse>(&creds, None).await?;
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
    xtream::api::<Vec<Category>>(&creds, Some(action)).await
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
#[tauri::command]
async fn get_contents(
    state: tauri::State<'_, AppState>,
    catalog: String,
    category_id: Option<String>,
) -> Result<Vec<Content>, String> {
    println!("[get_contents] ENTREE catalog={catalog} category_id={category_id:?}");
    {
        let cache = state
            .catalog
            .lock()
            .map_err(|e| format!("Cache vérouillé : {e}"))?;
        if let Some(entry) = cache.get(&catalog)
            && entry.is_fresh()
        {
            return Ok(filter(&entry.items, category_id.as_deref()));
        }
    }
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
    let items: Vec<Content> = match catalog.as_str() {
        "live" => xtream::api::<Vec<LiveContent>>(&creds, Some("get_live_streams"))
            .await?
            .into_iter()
            .map(Content::from)
            .collect(),
        "vod" => xtream::api::<Vec<VodContent>>(&creds, Some("get_vod_streams"))
            .await?
            .into_iter()
            .map(Content::from)
            .collect(),
        "serie" => xtream::api::<Vec<SeriesContent>>(&creds, Some("get_series"))
            .await?
            .into_iter()
            .map(Content::from)
            .collect(),
        other => return Err(format!("Catalogue inconnue : {other}")),
    };
    let result = filter(&items, category_id.as_deref());

    {
        let mut cache = state
            .catalog
            .lock()
            .map_err(|e| format!("État vérouillé : {e}"))?;
        cache.insert(
            catalog,
            CachedCatalog {
                items,
                fetched_at: SystemTime::now(),
            },
        )
    };
    Ok(result)
}

fn filter(items: &[Content], category_id: Option<&str>) -> Vec<Content> {
    match category_id {
        None => items.to_vec(),
        Some(id) => items
            .iter()
            .filter(|c| c.category_id.as_deref() == Some(id))
            .cloned()
            .collect(),
    }
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
            get_contents,
            get_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
