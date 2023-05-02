use std::error::Error;

use reqwest::{header, Client};

use crate::session_types;

async fn get_root_page(client: &Client) -> Result<String, Box<dyn Error>> {
    // let client = reqwest::Client::new();
    let url = "https://open.spotify.com/";
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "open.spotify.com".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.9".parse().unwrap());
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert("pragma", "no-cache".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Chromium\";v=\"112\", \"Google Chrome\";v=\"112\", \"Not:A-Brand\";v=\"99\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36".parse().unwrap());

    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}
fn extract_session_from_html(html: &str) -> Result<String, Box<dyn Error>> {
    let dom = tl::parse(&html, tl::ParserOptions::default())?;
    let parser = dom.parser();
    Ok(dom
        .get_element_by_id("session")
        .ok_or_else(|| "No element with id 'session' found")?
        .get(parser)
        .ok_or_else(|| "Element with id 'session' has no inner text")?
        .inner_text(parser)
        .into())
}
pub async fn get_new_session(client: &Client) -> Result<session_types::Session, Box<dyn Error>> {
    let html = get_root_page(client).await?;
    let session = extract_session_from_html(&html)?;
    let session: session_types::Session = serde_json::from_str(&session)?;
    Ok(session)
}
