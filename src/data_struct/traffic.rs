use std::collections::HashMap;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
pub struct Traffic {
    #[getset(get = "pub")]
    left: Option<u64>,
    #[getset(get = "pub")]
    bytes: Option<u32>,
    #[getset(get = "pub")]
    links: Option<u32>,
    #[getset(get = "pub")]
    limit: Option<u32>,
    #[getset(get = "pub")]
    #[serde(rename = "type")]
    resource_type: String,
    #[getset(get = "pub")]
    extra: Option<u32>,
    #[getset(get = "pub")]
    reset: Option<String>,
}
#[derive(Default, Debug, Clone, Getters)]
pub struct Traffics {
    #[getset(get = "pub")]
    pub(crate) result: HashMap<String, Traffic>,
}

/*
{
    "string": { // Host main domain
        "left": int, // Available bytes / links to use
        "bytes": int, // Bytes downloaded
        "links": int, // Links unrestricted
        "limit": int,
        "type": "string", // "links", "gigabytes", "bytes"
        "extra": int, // Additional traffic / links the user may have buy
        "reset": "string" // "daily", "weekly" or "monthly"
    },
    "string": {
        "left": int,
        "bytes": int,
        "links": int,
        "limit": int,
        "type": "string",
        "extra": int,
        "reset": "string"
    }
}
 */

#[derive(Default, Debug, Clone, Getters, Deserialize , Serialize)]
pub struct TrafficPeriod {
    #[getset(get = "pub")]
    host: HashMap<String, u64>,
    #[getset(get = "pub")]
    bytes: u64,
}

#[derive(Default, Debug, Clone, Getters)]
pub struct TrafficsPeriod {
    #[getset(get = "pub")]
    pub(crate) result : HashMap<String, TrafficPeriod>,
}

/*
{
    "YYYY-MM-DD": {
        "host": { // By Host main domain
            "string": int, // bytes downloaded on concerned host
            "string": int,
            "string": int,
            "string": int,
            "string": int,
            "string": int
        },
        "bytes": int // Total downloaded (in bytes) this day
    },
    "YYYY-MM-DD": {
        "host": {
            "string": int,
            "string": int,
            "string": int,
            "string": int,
            "string": int,
        },
        "bytes": int
    }
}
 */