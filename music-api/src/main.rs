use chrono::{Days, NaiveDateTime, Utc};
use clap::Parser;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::{Method, Url};
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

#[derive(thiserror::Error, Debug)]
pub enum MusicApiError {
    #[error("error while attempting to parse encoding key: {0}")]
    EncodingKeyError(#[from] jsonwebtoken::errors::Error),
    #[error("failed to create valid exp")]
    TimestampConversionError,
    #[error("failed to communicate with apple music api")]
    NetworkError(#[from] reqwest::Error),
}

#[derive(Serialize, Deserialize)]
struct AppleToken {
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Parser)]
struct Args {
    #[clap(long, env = "AUTH_KEY")]
    pub apple_auth_key: String,
    #[clap(long, env = "AUTH_ID")]
    pub apple_key_id: String,
    #[clap(long, env = "TEAM_ID")]
    pub apple_team_id: String,
    #[clap(long, env = "MUSIC_USER_TOKEN")]
    pub apple_music_user_token: String,
}

#[tokio::main]
async fn main() -> Result<(), MusicApiError> {
    let args = Args::parse();
    let encoding_key = jsonwebtoken::EncodingKey::from_ec_pem(&args.apple_auth_key.as_bytes())?;

    let content = AppleToken {
        iss: args.apple_team_id,
        iat: Utc::now().timestamp(),
        exp: Utc::now()
            .checked_add_days(Days::new(30))
            .ok_or_else(|| return MusicApiError::TimestampConversionError)?
            .timestamp(),
    };

    let token = encode(
        &Header {
            kid: Some(args.apple_key_id),
            alg: Algorithm::ES256,
            ..Default::default()
        },
        &content,
        &encoding_key,
    )?;

    let client = reqwest::Client::new();

    let req = client
        .get("https://api.music.apple.com/v1/me/recent/played/tracks?limit=3")
        .header("Music-User-Token", args.apple_music_user_token)
        .bearer_auth(token)
        .send()
        .await?
        .json::<RecentTracksResponse>()
        .await?;

    for track in req.data.iter() {
        println!(
            "Track: {}\nAlbum: {}\nArtwork: {}\nArtist: {}\n",
            track.attributes.name,
            track.attributes.album,
            track.attributes.artwork.url,
            track.attributes.artist
        )
    }

    Ok(())
}
