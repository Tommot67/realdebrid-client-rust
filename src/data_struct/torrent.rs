use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Default, Debug, Clone, Getters)]
pub struct TorrentFile {
    #[getset(get = "pub")]
    id: u32,
    #[getset(get = "pub")]
    path: String,
    #[getset(get = "pub")]
    bytes: u64,
    #[getset(get = "pub")]
    selected: u8,
}

#[allow(non_camel_case_types)]
pub enum ParamsTorrentFile {
    FROM_ALL,
    FROM_STRUCTS(Vec<TorrentFile>),
    FROM_IDS(Vec<String>)
}

/*
    "id": int,
    "path": "string", // Path to the file inside the torrent, starting with "/"
    "bytes": int,
    "selected": int // 0 or 1
 */

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
pub struct Torrent {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    filename: String,
    #[getset(get = "pub")]
    original_filename: Option<String>,
    #[getset(get = "pub")]
    hash: String,
    #[getset(get = "pub")]
    bytes: u64,
    #[getset(get = "pub")]
    original_bytes: Option<u64>,
    #[getset(get = "pub")]
    host: String,
    #[getset(get = "pub")]
    split: u32,
    #[getset(get = "pub")]
    progress: u16,
    #[getset(get = "pub")]
    status: String,
    #[getset(get = "pub")]
    added: String,
    #[getset(get = "pub")]
    files: Option<Vec<TorrentFile>>,
    #[getset(get = "pub")]
    links: Vec<String>,
    #[getset(get = "pub")]
    ended: Option<String>,
    #[getset(get = "pub")]
    speed: Option<u32>,
    #[getset(get = "pub")]
    seeders: Option<u32>,
}

#[derive(Default, Debug, Clone, Getters)]
pub struct Torrents {
    #[getset(get = "pub")]
    pub(crate) result: Vec<Torrent>,
    #[getset(get = "pub")]
    pub(crate) total_count: u64,
}

#[allow(non_camel_case_types)]
pub enum ParamsTorrent {
    FROM_STRUCT(Torrent),
    FROM_ADD(TorrentAdd),
    FROM_ID(String)
}

/*
    {
        "id": "string",
        "filename": "string",
        "original_filename": "string", // Original name of the torrent
        "hash": "string", // SHA1 Hash of the torrent
        "bytes": int, // Size of selected files only
        "original_bytes": int, // Total size of the torrent
        "host": "string", // Host main domain
        "split": int, // Split size of links
        "progress": int, // Possible values: 0 to 100
        "status": "downloaded", // Current status of the torrent: magnet_error, magnet_conversion, waiting_files_selection, queued, downloading, downloaded, error, virus, compressing, uploading, dead
        "added": "string", // jsonDate
        "files": [
            {
                "id": int,
                "path": "string", // Path to the file inside the torrent, starting with "/"
                "bytes": int,
                "selected": int // 0 or 1
            },
            {
                "id": int,
                "path": "string", // Path to the file inside the torrent, starting with "/"
                "bytes": int,
                "selected": int // 0 or 1
            }
        ],
        "links": [
            "string" // Host URL
        ],
        "ended": "string", // !! Only present when finished, jsonDate
        "speed": int, // !! Only present in "downloading", "compressing", "uploading" status
        "seeders": int // !! Only present in "downloading", "magnet_conversion" status
    }
*/

#[derive(Serialize, Deserialize, Default , Debug, Clone, Getters)]
pub struct TorrentCount {
    #[getset(get = "pub")]
    nb: u32,
    #[getset(get = "pub")]
    limit: u32,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
pub struct TorrentHost {
    #[getset(get = "pub")]
    host: String,
    #[getset(get = "pub")]
    max_file_size: u32,
}

#[allow(non_camel_case_types)]
pub enum ParamsTorrentHost {
    FROM_STRUCT(TorrentHost),
    FROM_HOST(String)
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
pub struct TorrentAdd {
    #[getset(get = "pub")]
    id: String,
    #[getset(get = "pub")]
    uri: String,
}