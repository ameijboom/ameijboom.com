use axum::{routing::get, Router};
use clap::Parser;
use models::AppleAuth;
use state::{AppleMusicUserToken, State};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub mod models;
pub mod routes;
pub mod state;
pub mod token;

#[derive(thiserror::Error, Debug)]
pub enum MusicApiError {
    #[error("error while attempting to parse encoding key: {0}")]
    EncodingKey(#[from] jsonwebtoken::errors::Error),
    #[error("failed to create valid exp")]
    TimestampConversion,
    #[error("failed to communicate with apple music api: {0}")]
    AppleMusic(#[from] reqwest::Error),
    #[error("failed to bind to hostname: {0}")]
    Network(#[from] std::io::Error),
}

#[derive(Parser)]
struct Args {
    #[clap(long, env = "APPLE_AUTH_KEY")]
    pub apple_auth_key: String,
    #[clap(long, env = "APPLE_KEY_ID")]
    pub apple_key_id: String,
    #[clap(long, env = "APPLE_TEAM_ID")]
    pub apple_team_id: String,
    #[clap(long, env = "APPLE_MUSIC_USER_TOKEN")]
    pub apple_music_user_token: String,
}

#[tokio::main]
async fn main() -> Result<(), MusicApiError> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let client = reqwest::Client::new();
    let encoding_key = jsonwebtoken::EncodingKey::from_ec_pem(&args.apple_auth_key.as_bytes())?;

    let state = State {
        apple_auth: AppleAuth {
            id: args.apple_key_id,
            key: encoding_key,
            team: args.apple_team_id,
        },
        apple_music_user_token: AppleMusicUserToken::new(args.apple_music_user_token),
        reqwest_client: client,
    };

    let app = Router::new()
        .route("/_health", get(health))
        .route("/api/v1/recent", get(routes::recent::v1::route))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    tracing::info!("started listening on 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn health() -> &'static str {
    return "OK";
}
