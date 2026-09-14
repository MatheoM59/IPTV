fn main() {
    dotenvy::dotenv().ok();
    if let Ok(cle) = std::env::var("GEMINI_API_KEY") {
        println!("cargo:rustc-env=GEMINI_API_KEY={cle}");
    }
    println!("cargo:rerun-if-changed=.env");
    tauri_build::build()
}
