use crate::mastodon::{Toot, TootTemplate};
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

#[derive(Template)]
#[template(path = "markdown.html")]
pub struct Markdown {
    pub title: String,
    pub url: Option<String>,
    pub root_toot: Toot,
    pub thread_children: Vec<Toot>,
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct Error {
    pub title: String,
    pub url: Option<String>,
    pub error: String,
}
