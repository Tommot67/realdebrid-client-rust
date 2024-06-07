use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default , Debug, Clone, Getters)]
pub struct User {
    #[getset(get = "pub")]
    id: u32,
    #[getset(get = "pub")]
    username: String,
    #[getset(get = "pub")]
    email: String,
    #[getset(get = "pub")]
    points: u32,
    #[getset(get = "pub")]
    locale: String,
    #[getset(get = "pub")]
    avatar: String,
    #[getset(get = "pub")]
    #[serde(rename = "type")]
    account_type: String,
    #[getset(get = "pub")]
    premium: u32,
    #[getset(get = "pub")]
    expiration: String,
}

/*
"id": int,
    "username": "string",
    "email": "string",
    "points": int, // Fidelity points
    "locale": "string", // User language
    "avatar": "string", // URL
    "type": "string", // "premium" or "free"
    "premium": int, // seconds left as a Premium user
    "expiration": "string" // jsonDate
 */