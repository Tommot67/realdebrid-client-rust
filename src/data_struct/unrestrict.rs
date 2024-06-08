use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
pub struct UnrestrictCheck {
    #[getset(get = "pub")]
    #[serde(default)]
    host: String,
    #[getset(get = "pub")]
    #[serde(default)]
    link: String,
    #[getset(get = "pub")]
    #[serde(default)]
    filename: String,
    #[getset(get = "pub")]
    #[serde(default)]
    filesize: u64,
    #[getset(get = "pub")]
    #[serde(default)]
    supported: u8,
}

/*
    {
        "host": "string", // Host main domain
        "link": "string",
        "filename": "string",
        "filesize": int,
        "supported": int
    }
 */

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
pub struct UnrestrictAlternative {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    filename: String,
    #[getset(get = "pub")]
    download: String,
    #[getset(get = "pub")]
    #[serde(rename = "type")]
    unrestrict_type: String
}

/*
{
    "id": "string",
    "filename": "string",
    "download": "string",
    "type": "string"
},
 */

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
#[allow(non_snake_case)]
pub struct Unrestrict {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    filename: String,
    #[getset(get = "pub")]
    mimeType: String,
    #[getset(get = "pub")]
    filesize: u64,
    #[getset(get = "pub")]
    link: String,
    #[getset(get = "pub")]
    host: String,
    #[getset(get = "pub")]
    chunks: u32,
    #[getset(get = "pub")]
    crc: u8,
    #[getset(get = "pub")]
    download: String,
    #[getset(get = "pub")]
    streamable: u8,
    #[getset(get = "pub")]
    #[serde(rename = "type")]
    unrestrict_type: Option<String>,
    #[getset(get = "pub")]
    alternative: Option<Vec<UnrestrictAlternative>>
}

/*
{
    "id": "string",
    "filename": "string",
    "filesize": int, // Filesize in bytes, 0 if unknown
    "link": "string", // Original link
    "host": "string", // Host main domain
    "chunks": int, // Max Chunks allowed
    "crc": int, // Disable / enable CRC check
    "download": "string", // Generated link
    "streamable": int, // Is the file streamable on website
    "type": "string", // Type of the file (in general, its quality)
    "alternative": [
        {
            "id": "string",
            "filename": "string",
            "download": "string",
            "type": "string"
        },
        {
            "id": "string",
            "filename": "string",
            "download": "string",
            "type": "string"
        }
    ]
}
 */