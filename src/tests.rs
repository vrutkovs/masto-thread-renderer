use std::vec;

use crate::mastodon::{self, MastoAccount};
use crate::mastodon::{Toot, TootContext};

use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

use base_url::BaseUrl;

#[test]
fn healthz() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(super::healthz)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "OK");
}

#[rocket::async_test]
async fn toot_id_from_url() {
    let result = mastodon::get_toot_id_from_url(
        BaseUrl::try_from("https://example.com/api/v1/statuses/123").unwrap(),
    )
    .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), String::from("123"));
}

#[rocket::async_test]
async fn toot_details() {
    let toot_id = "123";
    let toot = Toot {
        account: MastoAccount {
            id: "456".to_string(),
            url: "https://example.com".to_string(),
        },
        url: format!("https://example.com/note/{}", toot_id),
        in_reply_to_account_id: None,
        content: "hello".to_string(),
        media_attachments: vec![],
    };
    let toot_contents = serde_json::to_string(&toot).unwrap();

    let mut server = mockito::Server::new_async().await;
    let path = format!("/api/v1/statuses/{}", toot_id);
    let toot_url = BaseUrl::try_from(dbg!(format!("{}{}", server.url(), path).as_str())).unwrap();

    // Create a mock
    let _mock = server
        .mock("GET", path.as_str())
        .with_status(200)
        .with_body(toot_contents)
        .create_async()
        .await;

    let client = reqwest::Client::builder()
        .gzip(true)
        .connection_verbose(true)
        .user_agent("Masto-Thread-Renderer/0.0.1. Contact @vadim@vrutkovs.eu if misbehaving")
        .build()
        .unwrap();

    let result = mastodon::get_toot_details(&client, &toot_url).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), toot);

    let toot_url =
        BaseUrl::try_from(dbg!(format!("{}/no-such-url/", server.url()).as_str())).unwrap();
    let result = mastodon::get_toot_details(&client, &toot_url).await;
    assert!(result.is_err());
}

#[rocket::async_test]
async fn toot_context() {
    let toot_id = "123";
    let toot1 = Toot {
        account: MastoAccount {
            id: "456".to_string(),
            url: "https://example.com".to_string(),
        },
        url: "https://example.com/note/124".to_string(),
        in_reply_to_account_id: None,
        content: "hello ancestor".to_string(),
        media_attachments: vec![],
    };
    let toot2 = Toot {
        account: MastoAccount {
            id: "456".to_string(),
            url: "https://example.com".to_string(),
        },
        url: "https://example.com/note/125".to_string(),
        in_reply_to_account_id: None,
        content: "hello descendant".to_string(),
        media_attachments: vec![],
    };

    let toot_context = TootContext {
        ancestors: vec![toot1],
        descendants: vec![toot2],
    };
    let toot_contents = serde_json::to_string(&toot_context).unwrap();

    let mut server = mockito::Server::new_async().await;
    let context_path = format!("/api/v1/statuses/{}/context", toot_id);
    let toot_path = format!("/api/v1/statuses/{}", toot_id);
    let toot_url =
        BaseUrl::try_from(dbg!(format!("{}{}", server.url(), toot_path).as_str())).unwrap();

    // Create a mock
    let _mock = server
        .mock("GET", context_path.as_str())
        .with_status(200)
        .with_body(toot_contents)
        .create_async()
        .await;

    let client = reqwest::Client::builder()
        .gzip(true)
        .connection_verbose(true)
        .user_agent("Masto-Thread-Renderer/0.0.1. Contact @vadim@vrutkovs.eu if misbehaving")
        .build()
        .unwrap();

    let result = mastodon::get_toot_context(&client, &toot_url).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), toot_context);

    let toot_url =
        BaseUrl::try_from(dbg!(format!("{}/no-such-url/", server.url()).as_str())).unwrap();
    let result = mastodon::get_toot_context(&client, &toot_url).await;
    assert!(result.is_err());
}
