use reqwest;
use serde::{Deserialize, Serialize};
use crate::reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};

const CLIENT_ID: &str = "8a324da84ed94d5bb009339094e7a501";
const CLIENT_SECRET: &str = "ffa15d2f0e694043936bc46efe7d04f1";

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        // go check out her latest album. It's 🔥
        query = "Little Simz"
    );
    let auth_token = get_token().await;
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(AUTHORIZATION, auth_token.token_string())
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .expect("Error sending spotify api get request");

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<APIResponse>().await {
                Ok(parsed) => println!("Success! {:?}", parsed),
                Err(_) => println!("Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("We're unauthorized");
        }
        _ => (),
    }
}

#[derive(Deserialize, Debug)]
struct AccessToken {
    access_token: String,
    token_type: String,
    //expires_in: u32,
}

impl AccessToken {
    pub fn token_string(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }
}

async fn get_token() -> AccessToken {
    let client = reqwest::Client::new();
    let mut form = std::collections::HashMap::new();
    form.insert("grant_type", "client_credentials");
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .form(&form)
        //.json(&true)
        .send()
        .await
        .expect("Should get a response!");

    response.json::<AccessToken>().await.unwrap()
}

// Serialization for final output
#[derive(Serialize, Deserialize, Debug)]
struct ExternalUrls {
    spotify: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    name: String,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}
#[derive(Serialize, Deserialize, Debug)]
struct Items<T> {
    items: Vec<T>,
}
#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    tracks: Items<Track>,
}