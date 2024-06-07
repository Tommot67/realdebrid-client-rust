use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub(crate) struct AuthDevice {
    pub(crate) device_code: String,
    pub(crate) user_code: String,
    pub(crate) interval: u64
    ,
    pub(crate) expires_in: u64,
    pub(crate) verification_url: String,
    pub(crate) direct_verification_url: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub(crate) struct AuthCredential {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub(crate) struct AuthToken {
    pub(crate) access_token: String,
    pub(crate) expires_in: u64,
    pub(crate) token_type: String,
    pub(crate) refresh_token: String,
}
#[derive(Debug, Clone)]
pub(crate) struct AuthRefresh {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) refresh_token: String,
    pub(crate) auth_time: SystemTime,
    pub(crate) expires_in: u64,
}
