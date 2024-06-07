use std::collections::HashMap;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
pub struct HostStatus {
    #[getset(get = "pub")]
    status: String,
    #[getset(get = "pub")]
    check_time: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
pub struct Host {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    image: String,
    #[getset(get = "pub")]
    image_big: String,
    #[getset(get = "pub")]
    supported: Option<u8>,
    #[getset(get = "pub")]
    status: Option<String>,
    #[getset(get = "pub")]
    check_time: Option<String>,
    #[getset(get = "pub")]
    competitors_status: Option<HashMap<String, HostStatus>>,
}

#[derive(Default, Debug, Clone, Getters)]
pub struct Hosts {
    #[getset(get = "pub")]
    pub(crate) result: HashMap<String, Host>,
    #[getset(get = "pub")]
    pub(crate) with_status: bool,
}

/*
{
    "string": { // Host main domain
        "id": "string",
        "name": "string",
        "image": "string", // URL
        "supported": int, // 0 or 1
        "status": "string", // "up" / "down" / "unsupported"
        "check_time": "string", // jsonDate
        "competitors_status": {
            "string": { // Competitor domain
                "status": "string", // "up" / "down" / "unsupported"
                "check_time": "string" // jsonDate
            },
            "string": {
                "status": "string",
                "check_time": "string"
            },
            "string": {
                "status": "string",
                "check_time": "string"
            }
        }
    },
    "string": {
        "id": "string",
        "name": "string",
        "image": "string",
        "supported": int,
        "status": "string",
        "check_time": "string",
        "competitors_status": {
            "string": {
                "status": "string",
                "check_time": "string"
            },
            "string": {
                "status": "string",
                "check_time": "string"
            },
            "string": {
                "status": "string",
                "check_time": "string"
            }
        }
    }
}
 */