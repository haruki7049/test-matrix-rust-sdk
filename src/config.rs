use matrix_sdk::{
    SessionMeta, SessionTokens,
    authentication::matrix::MatrixSession,
    ruma::{DeviceId, owned_user_id},
};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    homeserver: Url,
    matrix_session: Option<MatrixSession>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            homeserver: Url::parse("https://matrix.org").expect("Failed to parse homeserver Url"),

            matrix_session: Some(MatrixSession {
                meta: SessionMeta {
                    user_id: owned_user_id!("@example:localhost"),
                    device_id: DeviceId::new(),
                },

                tokens: SessionTokens {
                    access_token: String::new(),
                    refresh_token: None,
                },
            }),
        }
    }
}

impl Configuration {
    pub fn homeserver(&self) -> Url {
        self.homeserver.clone()
    }

    pub fn matrix_session(&self) -> Option<MatrixSession> {
        self.matrix_session.clone()
    }
}
