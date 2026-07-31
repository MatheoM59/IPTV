use crate::xtream::{ApiResponse, AppState, Category, Credentials};
use std::sync::Mutex;
mod xtream;

#[tauri::command]
fn get_account_info(
    state: tauri::State<AppState>,
    host: String,
    username: String,
    password: String,
) -> Result<ApiResponse, String> {
    let account = xtream::api::<ApiResponse>(&host, &username, &password, None)?;
    if account.user_info.auth != 1 {
        return Err("Erreur de connection".to_string());
    }
    if !account.user_info.status.eq_ignore_ascii_case("active") {
        return Err(format!(
            "Abonnement non actif : {}",
            account.user_info.status
        ));
    }
    let mut guard = state
        .credentials
        .lock()
        .map_err(|e| format!("État vérouillé : {e}"))?;
    *guard = Some(Credentials {
        host,
        username,
        password,
    });
    Ok(account)
}

#[tauri::command]
fn get_categories(state: tauri::State<AppState>, catalog: String) -> Result<Vec<Category>, String> {
    let guard = state
        .credentials
        .lock()
        .map_err(|e| format!("État vérouillé : {e}"))?;
    let creds = guard.as_ref().ok_or("Aucune connection active")?;
    let action = match catalog.as_str() {
        "live" => "get_live_categories",
        "vod" => "get_movie_categories",
        "serie" => "get_series_categories",
        other => return Err(format!("Catalogue inconnue : {other}")),
    };
    xtream::api::<Vec<Category>>(&creds.host, &creds.username, &creds.password, Some(action))
}

#[tauri::command]
fn get_status(state: tauri::State<AppState>) -> Result<bool, String> {
    let guard = state
        .credentials
        .lock()
        .map_err(|e| format!("État vérouillé : {e}"))?;
    Ok(guard.is_some())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            credentials: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_account_info,
            get_categories,
            get_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
