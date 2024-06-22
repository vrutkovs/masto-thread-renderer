use crate::errors::RenderError;
use crate::mastodon;
use crate::templates;
use anyhow::{Context, Result};
use base_url::{BaseUrl, TryFrom};
use rocket::State;

#[get("/")]
pub async fn index() -> templates::Index {
    templates::Index {
        title: "Index".to_string(),
        url: None,
    }
}

#[get("/thread?<url>")]
pub async fn thread(
    url: String,
    client: &State<reqwest::Client>,
) -> anyhow::Result<templates::Thread, RenderError> {
    let toot_url = BaseUrl::try_from(url.as_str())
        .map_err(|e| anyhow!("{:?}", e))
        .context("fetching initial toot")?;
    let root_toot =
        mastodon::get_toot_embed_code(toot_url.clone()).context("fetching toot embed code")?;

    let toot_details = mastodon::get_toot_details(client, &toot_url)
        .await
        .context("fetching toot details")?;
    let thread_children: Vec<mastodon::TootTemplate> =
        mastodon::get_children(client, &toot_url, &toot_details.account)
            .await
            .context("fetching toot replies")?
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
pub async fn markdown(
    url: String,
    client: &State<reqwest::Client>,
) -> Result<templates::Markdown, RenderError> {
    let toot_url = BaseUrl::try_from(url.as_str())
        .map_err(|e| anyhow!("{:?}", e))
        .context("fetching initial toot")?;

    let root_toot = mastodon::get_toot_details(client, &toot_url)
        .await
        .context("fetching toot details")?;
    let thread_children: Vec<mastodon::Toot> =
        mastodon::get_children(client, &toot_url, &root_toot.account)
            .await
            .context("fetching toot replies")?;
    Ok(templates::Markdown {
        title: "Markdown".to_string(),
        url: Some(url.clone()),
        root_toot,
        thread_children,
    })
}
