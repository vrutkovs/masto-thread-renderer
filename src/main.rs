#[macro_use]
extern crate rocket;
#[macro_use]
extern crate anyhow;
use base_url::{BaseUrl, TryFrom};
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::response::status::Custom;

mod mastodon;
mod templates;

#[get("/")]
async fn index() -> templates::Index {
    templates::Index {
        title: "Index".to_string(),
        url: None,
    }
}

#[get("/thread?<url>")]
async fn thread(url: String) -> Result<templates::Thread, Custom<String>> {
    let toot_url = BaseUrl::try_from(url.as_str())
        .map_err(|e| Custom(Status::InternalServerError, format!("{:?}", e)))?;
    let root_toot = mastodon::get_toot_embed_code(toot_url.clone())
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    let author_url = mastodon::get_toot_author(toot_url.clone())
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    let thread_children = mastodon::get_children(toot_url.clone(), author_url)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    Ok(templates::Thread {
        title: "Thread".to_string(),
        url: Some(url.clone()),
        root_toot,
        thread_children,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, thread])
        .mount("/public", FileServer::from("public"))
}
