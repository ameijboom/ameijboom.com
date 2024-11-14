pub mod v1 {
    use axum::{extract::State, Json};
    use http_api_problem::ApiError;
    use reqwest::StatusCode;
    use std::time::Instant;

    use crate::{
        models::{AppleAuth, RecentTrack, RecentTracksResponse},
        state::AppleMusicUserToken,
        token::get_apple_token,
    };

    pub async fn route(
        State(auth): State<AppleAuth>,
        State(client): State<reqwest::Client>,
        State(music_user_token): State<AppleMusicUserToken>,
    ) -> Result<Json<Vec<RecentTrack>>, ApiError> {
        let token = get_apple_token(&auth.team, &auth.id, &auth.key)
            .await
            .map_err(|e| {
                tracing::error!(error = &e as &dyn std::error::Error, "failed to create jwt");
                ApiError::builder(StatusCode::INTERNAL_SERVER_ERROR)
                    .title("jwt error")
                    .source_in_a_box(e)
                    .finish()
            })?;

        let start = Instant::now();
        tracing::debug!("start fetching data from apple music");
        let req = client
            .get("https://api.music.apple.com/v1/me/recent/played/tracks?limit=3")
            .header("Music-User-Token", music_user_token.to_string())
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(
                    error = &e as &dyn std::error::Error,
                    "failed to fetch recent tracks"
                );
                ApiError::builder(StatusCode::INTERNAL_SERVER_ERROR)
                    .title("apple music api error")
                    .source_in_a_box(e)
                    .finish()
            })?
            .json::<RecentTracksResponse>()
            .await
            .map_err(|e| {
                tracing::error!(
                    error = &e as &dyn std::error::Error,
                    "failed to convert objects to json"
                );
                ApiError::builder(StatusCode::INTERNAL_SERVER_ERROR)
                    .title("object conversion error")
                    .source_in_a_box(e)
                    .finish()
            })?;
        tracing::debug!({elapsed = start.elapsed().as_millis()}, "finished retrieving data from apple music");
        Ok(Json(req.data))
    }
}
