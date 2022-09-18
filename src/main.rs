#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;

use rocket::fs::FileServer;

mod mastodon;
mod routes;
mod templates;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();
    let public_files_path = figment
        .extract_inner("public_files_path")
        .unwrap_or("public");
    rocket
        .mount(
            "/",
            routes![routes::index, routes::thread, routes::markdown],
        )
        .mount("/public", FileServer::from(public_files_path))
}
