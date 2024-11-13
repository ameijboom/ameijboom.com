use chrono::{Days, Utc};
use jsonwebtoken::{encode, Algorithm, Header};

use crate::{models::AppleToken, MusicApiError};

pub async fn get_apple_token(
    team_id: &str,
    key_id: &str,
    encoding_key: &jsonwebtoken::EncodingKey,
) -> Result<String, MusicApiError> {
    let content = AppleToken {
        iss: team_id.to_string(),
        iat: Utc::now().timestamp(),
        exp: Utc::now()
            .checked_add_days(Days::new(1))
            .ok_or_else(|| return MusicApiError::TimestampConversion)?
            .timestamp(),
    };

    let token = encode(
        &Header {
            kid: Some(key_id.to_string()),
            alg: Algorithm::ES256,
            ..Default::default()
        },
        &content,
        &encoding_key,
    )?;

    Ok(token)
}
