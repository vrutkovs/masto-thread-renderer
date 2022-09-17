#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;

mod templates;

#[get("/")]
fn index() -> templates::Index {
    templates::Index {
        title: "Index".to_string(),
        url: None,
    }
}

#[get("/thread?<url>")]
fn thread(url: &str) -> templates::Thread {
    templates::Thread {
        title: "Thread".to_string(),
        url: Some(url.to_string()),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, thread])
        .mount("/public", FileServer::from("public"))
}
