use std::fmt;
use std::sync::Arc;

use axum::extract::FromRef;
use jsonwebtoken::EncodingKey;

use crate::models::AppleAuth;

macro_rules! UniqueString {
    ($($name:ident),+) => {
        $(#[derive(Clone)]
        pub struct $name(Arc<String>);

        impl $name {
            pub fn new(value: String) -> Self {
                Self(Arc::new(value))
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        })+
    };
}

UniqueString!(AppleMusicUserToken);

#[derive(Clone)]
pub struct State {
    pub apple_auth: AppleAuth,
    pub apple_music_user_token: AppleMusicUserToken,
    pub reqwest_client: reqwest::Client,
}

impl FromRef<State> for AppleAuth {
    fn from_ref(state: &State) -> Self {
        state.apple_auth.clone()
    }
}

impl FromRef<State> for AppleMusicUserToken {
    fn from_ref(state: &State) -> Self {
        state.apple_music_user_token.clone()
    }
}

impl FromRef<State> for reqwest::Client {
    fn from_ref(state: &State) -> Self {
        state.reqwest_client.clone()
    }
}
