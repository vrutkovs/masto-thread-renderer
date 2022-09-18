use crate::mastodon;
use crate::templates;
use base_url::{BaseUrl, TryFrom};
use rocket::http::Status;
use rocket::response::status::Custom;

#[get("/")]
pub async fn index() -> templates::Index {
    templates::Index {
        title: "Index".to_string(),
        url: None,
    }
}

#[get("/thread?<url>")]
pub async fn thread(url: String) -> Result<templates::Thread, Custom<String>> {
    let toot_url = BaseUrl::try_from(url.as_str())
        .map_err(|e| Custom(Status::InternalServerError, format!("{:?}", e)))?;
    let root_toot = mastodon::get_toot_embed_code(toot_url.clone())
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    let toot_details = mastodon::get_toot_details(toot_url.clone())
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    let thread_children: Vec<mastodon::TootTemplate> =
        mastodon::get_children(toot_url.clone(), toot_details.account)
            .await
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
            .iter()
            // Filter out toots with invalid URL
            .filter_map(|t| BaseUrl::try_from(t.url.as_str()).ok())
            .filter_map(|u| mastodon::get_toot_embed_code(u).ok())
            .collect::<Vec<mastodon::TootTemplate>>();
    Ok(templates::Thread {
        title: "Thread".to_string(),
        url: Some(url.clone()),
        root_toot,
        thread_children,
    })
}

#[get("/markdown?<url>")]
pub async fn markdown(url: String) -> Result<templates::Markdown, Custom<String>> {
    let toot_url = BaseUrl::try_from(url.as_str())
        .map_err(|e| Custom(Status::InternalServerError, format!("{:?}", e)))?;
    let root_toot = mastodon::get_toot_details(toot_url.clone())
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    let thread_children: Vec<mastodon::Toot> =
        mastodon::get_children(toot_url.clone(), root_toot.clone().account)
            .await
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    Ok(templates::Markdown {
        title: "Markdown".to_string(),
        url: Some(url.clone()),
        root_toot,
        thread_children,
    })
}
