use crate::mastodon::TootTemplate;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub title: String,
    pub url: Option<String>,
}

#[derive(Template)]
#[template(path = "thread.html")]
pub struct Thread {
    pub title: String,
    pub url: Option<String>,
    pub root_toot: TootTemplate,
    pub thread_children: Vec<TootTemplate>,
}
