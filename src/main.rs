#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::response::status::BadRequest;

mod mastodon;
mod templates;

#[get("/")]
fn index() -> templates::Index {
    templates::Index {
        title: "Index".to_string(),
        url: None,
    }
}

#[get("/thread?<url>")]
fn thread(url: String) -> Result<templates::Thread, BadRequest<String>> {
    let root_toot =
        mastodon::get_toot_embed_code(url.clone()).map_err(|e| BadRequest(Some(e.to_string())))?;
    let thread_children =
        mastodon::get_children(url.clone()).map_err(|e| BadRequest(Some(e.to_string())))?;
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
