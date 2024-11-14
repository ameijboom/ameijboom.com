use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackArt {
    #[serde(rename = "bgColor")]
    pub colour: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackAttributes {
    #[serde(rename = "albumName")]
    pub album: String,
    #[serde(rename = "artistName")]
    pub artist: String,
    pub name: String,
    pub url: String,
    pub artwork: TrackArt,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct RecentTrack {
    pub attributes: TrackAttributes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentTracksResponse {
    pub data: Vec<RecentTrack>,
}

#[derive(Serialize, Deserialize)]
pub struct AppleToken {
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone)]
pub struct AppleAuth {
    pub id: String,
    pub key: EncodingKey,
    pub team: String,
}
