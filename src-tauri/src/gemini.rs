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
const CONSIGNE: &str =
    "Tu identifies des films et série a partir des descriptions qui te sont envoyé.
 Ne propose que des oeuvre dont tu es résonnablement sur de l'existence.
  Si tu ne trouve rien de plausible, renvoie une liste vide plutot que d'inventer.
   N'invente jamais un titre francais, si tu ne le connais pas , reprend le titre original. ";

pub async fn chercher(requete: &str, nb: u8) -> Result<Vec<Oeuvre>, String> {
    let cle = GEMINI_KEY.ok_or("Aucune clé API configuré")?;
    let url =
        format!("https://generativelanguage.googleapis.com/v1beta/models/{MODEL}:generateContent");
    let corps = json!({
        "systemInstruction": { "parts": [{ "text": CONSIGNE }] },
        "contents": [{
            "parts": [{ "text": format!("{requete}\n\nDonne au maximum {nb} résultats.") }]
        }],
        "generationConfig": {
            "responseMimeType": "application/json",
            "responseSchema": {
                "type": "object",
                "properties": {
                    "resultats": {
                        "type": "array",
                        "description": "Œuvres correspondant à la demande, de la plus probable à la moins probable",
                        "items": {
                            "type": "object",
                            "properties": {
                                "titre_fr": { "type": "string", "description": "Titre de distribution français" },
                                "titre_original": { "type": "string", "description": "Titre dans la langue d'origine" },
                                "annee": { "type": "integer", "description": "Année de sortie ou de première diffusion" },
                                "type_oeuvre": { "type": "string", "enum": ["film", "serie"], "description": "Nature de l'œuvre" }
                            },
                            "required": ["titre_fr", "titre_original", "annee", "type_oeuvre"]
                        }
                    }
                },
                "required": ["resultats"]
            }
        }
    });
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
    let enveloppe =
        serde_json::from_str::<GeminiResponse>(&body).map_err(|e| format!("Erreur : {e}"))?;
    let candidates = enveloppe.candidates.first().ok_or("Reponse vide ")?;
    let part = candidates.content.parts.first().ok_or("Aucun contenue")?;
    let texte = part.text.clone().ok_or("Aucun text")?;
    let resultats: Resultats =
        serde_json::from_str(&texte).map_err(|e| format!("Parse résultats : {e}"))?;
    Ok(resultats.resultats)
}
