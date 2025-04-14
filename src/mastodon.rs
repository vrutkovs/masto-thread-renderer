use std::cmp::PartialEq;
use std::vec;

use crate::anyhow::Context;
use crate::anyhow::Result as Fallible;
use base_url::BaseUrl;
use html2md::parse_html;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct TootTemplate {
    pub embed_url: String,
    pub embed_js: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MastoAccount {
    pub id: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Toot {
    pub account: MastoAccount,
    pub url: String,
    pub in_reply_to_account_id: Option<String>,
    pub content: String,
    pub media_attachments: Vec<MediaAttachement>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MediaAttachement {
    #[serde(rename = "type")]
    pub media_type: String,
    pub url: String,
    pub description: Option<String>,
}

impl Toot {
    pub fn media_attachments_to_markdown(&self) -> String {
        self.media_attachments
            .clone()
            .into_iter()
            .filter(|m| m.media_type == "image")
            .map(|m| {
                format!(
                    "![{0}]({1})",
                    m.description.unwrap_or("No alt text".to_string()),
                    m.url,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn content_to_markdown(&self) -> String {
        let content = parse_html(self.content.as_str());
        let media_attachments = self.media_attachments_to_markdown();
        format!("{content}\n{media_attachments}")
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TootContext {
    pub ancestors: Vec<Toot>,
    pub descendants: Vec<Toot>,
}

pub fn get_toot_embed_code(toot_url: BaseUrl) -> Fallible<TootTemplate> {
    let mut embed_url = toot_url.clone();
    embed_url.set_path(format!("{}/embed", embed_url.path()).as_str());
    let mut embed_js = toot_url;
    embed_js.set_path("/embed.js");
    Ok(TootTemplate {
        embed_url: embed_url.to_string(),
        embed_js: embed_js.to_string(),
    })
}

pub async fn get_toot_id_from_url(toot_url: BaseUrl) -> Fallible<String> {
    // Last section of the URL is status ID
    toot_url
        .path_segments()
        .next_back()
        .ok_or("invalid URL")
        .map(|s| s.to_string())
        .map_err(|e| anyhow!(e.to_string()))
}

pub async fn get_toot_details(client: &reqwest::Client, toot_url: &BaseUrl) -> Fallible<Toot> {
    let toot_id = get_toot_id_from_url(toot_url.clone())
        .await
        .context("fetching toot id")?;
    let mut toot_details_url = toot_url.clone();
    toot_details_url.set_path(format!("/api/v1/statuses/{}", toot_id).as_str());
    let details_url = dbg!(toot_details_url.to_string());
    let response = client
        .get(details_url)
        .send()
        .await
        .context("fetching toot")?;

    let response_text = dbg!(response.text().await.unwrap());
    dbg!(serde_json::from_str::<Toot>(&response_text).map_err(|e| anyhow!(e)))
}

pub async fn get_toot_context(
    client: &reqwest::Client,
    toot_url: &BaseUrl,
) -> Fallible<TootContext> {
    // Last section of the URL is status ID
    let toot_id = toot_url
        .path_segments()
        .next_back()
        .ok_or("invalid URL")
        .map_err(|e| anyhow!(e.to_string()))
        .context("fetching toot id")?;
    let mut toot_context_url = toot_url.clone();
    toot_context_url.set_path(format!("/api/v1/statuses/{}/context", toot_id).as_str());
    let response = client
        .get(toot_context_url.to_string())
        .send()
        .await
        .context("fetching toot context")?;
    let response_text = dbg!(response.text().await.unwrap());
    dbg!(serde_json::from_str::<TootContext>(&response_text).map_err(|e| anyhow!(e)))
}

pub async fn get_children(
    client: &reqwest::Client,
    toot_url: &BaseUrl,
    author: &MastoAccount,
) -> Fallible<Vec<Toot>> {
    let mut found_first_toot = false;
    let mut current_toot = toot_url.clone();
    let mut result: Vec<Toot> = vec![];
    loop {
        let toot_context = get_toot_context(client, &current_toot)
            .await
            .context("fetching toot context")?;

        if !found_first_toot {
            let maybe_first_toot = toot_context.ancestors.first();
            if let Some(first_toot) = maybe_first_toot {
                current_toot = BaseUrl::try_from(first_toot.url.as_ref())
                    .map_err(|e| anyhow!("{:?}", e))
                    .context("fetching last toot")?;
                continue;
            } else {
                found_first_toot = true
            }
        }

        let mut descendants: Vec<Toot> = toot_context
            .descendants
            .iter()
            // Filter out replies from other users or from author to other users
            .filter(|t| {
                t.account.url == author.url && t.in_reply_to_account_id == Some(author.clone().id)
            })
            .cloned()
            .collect();
        if descendants.is_empty() {
            break;
        }
        // Get last toot and check for its descendants too
        let last_toot = descendants.last().unwrap().clone();
        result.append(&mut descendants);
        current_toot = BaseUrl::try_from(last_toot.url.as_ref())
            .map_err(|e| anyhow!("{:?}", e))
            .context("fetching last toot")?;
    }
    Ok(result)
}
