mod xtream;
use crate::xtream::ApiResponse;

#[tauri::command]
fn get_account_info(
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
    Ok(account)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_account_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
