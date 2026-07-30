mod xtream;
use std::env;

use crate::xtream::ApiResponse;
use crate::xtream::Category;
use crate::xtream::StreamKind;
fn main() {
    let host = "http://pl-ott.com";
    let username = env::var("XTREAM_USER").expect("Variable XTREAM_USER absente");
    let password = env::var("XTREAM_PASS").expect("Variable XTREAM_PASS absente");
    let account: ApiResponse = match xtream::api(host, &username, &password, None) {
        Ok(a) => a,
        Err(e) => {
            println!("Erreur : {e}");
            return;
        }
    };

    if account.user_info.auth != 1 {
        println!("Login failed");
        return;
    }
    let user_status = account.user_info.status;
    if !user_status.eq_ignore_ascii_case("active") {
        println!("Account {user_status}");
        return;
    }
    let cats: Vec<Category> =
        match xtream::api(host, &username, &password, Some("get_vod_categories")) {
            Ok(a) => a,
            Err(e) => {
                println!("Erreur : {e}");
                return;
            }
        };
    let url_film =
        xtream::build_stream_url(host, &username, &password, StreamKind::Movie, 412927, "mkv");
    let url_live =
        xtream::build_stream_url(host, &username, &password, StreamKind::Live, 502494, "ts");
    let url_serie =
        xtream::build_stream_url(host, &username, &password, StreamKind::Series, 1, "mkv");
    println!("Account {user_status} and connected");

    println!("{cats:#?}");

    println!("live  : {url_serie}");
}
