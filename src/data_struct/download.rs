use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
pub struct Download {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    filename: String,
    #[getset(get = "pub")]
    mime_type: Option<String>,
    #[getset(get = "pub")]
    filesize: u64,
    #[getset(get = "pub")]
    link: String,
    #[getset(get = "pub")]
    host: String,
    #[getset(get = "pub")]
    host_icon: Option<String>,
    #[getset(get = "pub")]
    chunks: u32,
    #[getset(get = "pub")]
    download: String,
    #[getset(get = "pub")]
    streamable: u8,
    #[getset(get = "pub")]
    generated: String,
    #[getset(get = "pub")]
    #[serde(rename = "type")]
    file_type: Option<String>,
}

#[derive(Default, Debug, Clone, Getters)]
pub struct Downloads {
    #[getset(get = "pub")]
    pub(crate) result: Vec<Download>,
    #[getset(get = "pub")]
    pub(crate) total_count: u64,
}

#[allow(non_camel_case_types)]
pub enum ParamsDownload {
    FROM_STRUCT(Download),
    FROM_ID(String)
}

/*
[
    {
        "id": "string",
        "filename": "string",
        "mimeType": "string", // Mime Type of the file, guessed by the file extension
        "filesize": int, // bytes, 0 if unknown
        "link": "string", // Original link
        "host": "string", // Host main domain
        "chunks": int, // Max Chunks allowed
        "download": "string", // Generated link
        "generated": "string" // jsonDate
    },
    {
        "id": "string",
        "filename": "string",
        "mimeType": "string",
        "filesize": int,
        "link": "string",
        "host": "string",
        "chunks": int,
        "download": "string",
        "generated": "string",
        "type": "string" // Type of the file (in general, its quality)
    },
    {
        id": "DU3RBZORK46LI",
        "filename": "nubiles.23.11.28.monica.storm.so.good.mp4",
        "mimeType": "video/mp4",
        "filesize": 502569825,
        "link": "https://real-debrid.com/d/LBAPSXMRUQLJQHVHJWPS7PD2ZI",
        "host": "real-debrid.com",
        "host_icon": "https://fcdn.real-debrid.com/0822/images/hosters/realdebrid.png",
        "chunks": 32,
        "download": "https://71.download.real-debrid.com/d/DU3RBZORK46LI/nubiles.23.11.28.monica.storm.so.good.mp4",
        "streamable": 1,
        "generated": "2024-05-07T00:28:35.000Z"
    }
]
 */