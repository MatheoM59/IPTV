use serde::Deserialize;
use serde_json::json;
use std::time::Duration;

/// Typage et structure

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidates>,
}

#[derive(Debug, Deserialize)]
struct Candidates {
    content: GeminiContent,
}

#[derive(Debug, Deserialize)]
struct GeminiContent {
    parts: Vec<Part>,
}

#[derive(Debug, Deserialize)]
struct Part {
    text: Option<String>,
}
#[derive(Debug, Deserialize)]
struct Resultats {
    resultats: Vec<Oeuvre>,
}
#[derive(Debug, Deserialize)]
pub struct Oeuvre {
    pub titre_fr: String,
    pub titre_original: String,
    pub annee: u16,
    pub type_oeuvre: String,
}

const GEMINI_KEY: Option<&str> = option_env!("GEMINI_API_KEY");
const MODEL: &str = "gemini-3.6-flash";

/// Function
pub async fn chercher(requete: &str, nb: u8) -> Result<Vec<Oeuvre>, String> {
    let cle = GEMINI_KEY.ok_or("Aucune clé API configuré")?;
    let url =
        format!("https://generativelanguage.googleapis.com/v1beta/models/{MODEL}:generateContent");
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Client : {e}"))?;
    let resp = client
        .post(&url)
        .header("x-goog-api-key", cle)
        .json(&corps)
        .send()
        .await
        .map_err(|e| format!("Envoi : {e}"))?;
    let statut = resp.status();
    if statut == 429 || statut == 503 {
        return Err("Service momentanément indisponible , réessaie".to_string());
    }
    if !statut.is_success() {
        return Err(format!("Requête refusée (status : {statut})"));
    }
    let body = resp.text().await.map_err(|e| format!("Corps : {e}"))?;
}
