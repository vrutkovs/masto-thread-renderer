use crate::anyhow::Result as Fallible;
use base_url::BaseUrl;

#[derive(Clone)]
pub struct TootTemplate {
    pub embed_url: String,
    pub embed_js: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct MastoAccount {
    pub id: String,
    pub url: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct Toot {
    pub account: MastoAccount,
    pub url: String,
    pub in_reply_to_account_id: Option<String>,
    pub content: String,
}

#[derive(serde::Deserialize)]
pub struct TootContext {
    pub ancestors: Vec<Toot>,
    pub descendants: Vec<Toot>,
}

pub fn get_toot_embed_code(toot_url: BaseUrl) -> Fallible<TootTemplate> {
    let mut embed_url = toot_url.clone();
    embed_url.set_path(format!("{}/embed", embed_url.path()).as_str());
    let mut embed_js = toot_url.clone();
    embed_js.set_path("/embed.js");
    return Ok(TootTemplate {
        embed_url: embed_url.to_string(),
        embed_js: embed_js.to_string(),
    });
}

pub async fn get_toot_id_from_url(toot_url: BaseUrl) -> Fallible<String> {
    // Last section of the URL is status ID
    toot_url
        .path_segments()
        .last()
        .ok_or("invalid URL")
        .map(|s| s.to_string())
        .map_err(|e| anyhow!(e.to_string()))
}

pub async fn get_toot_details(toot_url: BaseUrl) -> Fallible<Toot> {
    let toot_id = get_toot_id_from_url(toot_url.clone()).await?;
    let mut toot_details_url = toot_url.clone();
    toot_details_url.make_host_only();
    toot_details_url.set_path(format!("/api/v1/statuses/{}", toot_id).as_str());
    let client = reqwest::Client::new();
    Ok(client
        .get(toot_details_url.to_string())
        .send()
        .await?
        .json::<Toot>()
        .await?)
}

pub async fn get_children(toot_url: BaseUrl, author: MastoAccount) -> Fallible<Vec<Toot>> {
    // Last section of the URL is status ID
    let toot_id = toot_url
        .path_segments()
        .last()
        .ok_or("invalid URL")
        .map_err(|e| anyhow!(e.to_string()))?;
    let mut toot_context_url = toot_url.clone();
    toot_context_url.make_host_only();
    toot_context_url.set_path(format!("/api/v1/statuses/{}/context", toot_id).as_str());
    let client = reqwest::Client::new();
    let toot_context = client
        .get(toot_context_url.to_string())
        .send()
        .await?
        .json::<TootContext>()
        .await?;
    Ok(toot_context
        .descendants
        .iter()
        // Filter out replies from other users or from author to other users
        .filter(|t| {
            t.account.url == author.url && t.in_reply_to_account_id == Some(author.clone().id)
        })
        .map(|t| t.clone())
        .collect())
}
