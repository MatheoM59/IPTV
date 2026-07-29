mod xtream;
use std::env;

use crate::xtream::ApiResponse;
use crate::xtream::Category;
fn main() {
    let host = "http://pl-ott.com";
    let username = env::var("XTREAM_USER").expect("Variable XTREAM_USER absente");
    let password = env::var("XTREAM_PASS").expect("Variable XTREAM_PASS absente");
    let account: ApiResponse = xtream::api(host, &username, &password, None);

    let cats: Vec<Category> = xtream::api(host, &username, &password, Some("get_vod_categories"));
    if account.user_info.auth != 1 {
        println!("Login failed");
        return;
    }
    let user_status = account.user_info.status;
    if !user_status.eq_ignore_ascii_case("active") {
        println!("Account {user_status}");
        return;
    }
    println!("Account {user_status} and connected");
    println!("{cats:#?}")
}
