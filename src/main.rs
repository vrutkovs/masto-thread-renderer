#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;

mod templates;

#[get("/")]
fn index() -> templates::Index {
    templates::Index {
        title: "Index".to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("public"))
}
