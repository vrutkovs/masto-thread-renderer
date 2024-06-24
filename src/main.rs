#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;

use log::{info, log_enabled, Level};
use rocket::fs::FileServer;

mod errors;
mod mastodon;
mod routes;
mod templates;
#[cfg(test)]
mod tests;

#[rocket::get("/healthz")]
fn healthz() -> String {
    "OK".to_string()
}

#[launch]
fn rocket() -> _ {
    env_logger::init();

    if log_enabled!(Level::Info) {
        info!("Running commit {}", env!("GIT_HASH"))
    }
    let rocket = rocket::build();
    let figment = rocket.figment();
    let public_files_path = figment
        .extract_inner("public_files_path")
        .unwrap_or("public");
    let client = reqwest::Client::builder()
        .gzip(true)
        .connection_verbose(true)
        .user_agent("Masto-Thread-Renderer/0.0.1. Contact @vadim@vrutkovs.eu if misbehaving")
        .build()
        .unwrap();

    rocket
        .manage(client)
        .mount(
            "/",
            routes![healthz, routes::index, routes::thread, routes::markdown],
        )
        .mount("/public", FileServer::from(public_files_path))
}
