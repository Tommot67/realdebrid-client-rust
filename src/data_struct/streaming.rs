use std::collections::HashMap;
use getset::Getters;
use serde::{Deserialize, Serialize};
use crate::data_struct::download::Download;
use crate::data_struct::streaming::VecOrHashMap::{Right};
use crate::data_struct::unrestrict::Unrestrict;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Getters)]
#[allow(non_snake_case)]
pub struct StreamingTranscode {
    #[getset(get = "pub")]
    apple: HashMap<String, String>,
    #[getset(get = "pub")]
    dash: HashMap<String, String>,
    #[getset(get = "pub")]
    liveMP4: HashMap<String, String>,
    #[getset(get = "pub")]
    h264WebM: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum VecOrHashMap {
    Left(Vec<String>),
    Right(HashMap<String, SubtitleDetails>)
}

impl Default for VecOrHashMap {
    fn default() -> Self {
        Right(HashMap::new())
    }
}

#[derive(Debug , Default, Serialize, Deserialize, Clone, Getters)]
pub struct MediaDetails {
    #[getset(get = "pub")]
    video: Option<HashMap<String, VideoDetails>>,
    #[getset(get = "pub")]
    audio: Option<HashMap<String, AudioDetails>>,
    #[getset(get = "pub")]
    subtitles: VecOrHashMap,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Getters)]
pub struct VideoDetails {
    #[getset(get = "pub")]
    stream: String,
    #[getset(get = "pub")]
    lang: String,
    #[getset(get = "pub")]
    lang_iso: String,
    #[getset(get = "pub")]
    codec: String,
    #[getset(get = "pub")]
    colorspace: String,
    #[getset(get = "pub")]
    width: i32,
    #[getset(get = "pub")]
    height: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Getters)]
pub struct AudioDetails {
    #[getset(get = "pub")]
    stream: String,
    #[getset(get = "pub")]
    lang: String,
    #[getset(get = "pub")]
    lang_iso: String,
    #[getset(get = "pub")]
    codec: String,
    #[getset(get = "pub")]
    sampling: u32,
    #[getset(get = "pub")]
    channels: f32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Getters)]
pub struct SubtitleDetails {
    #[getset(get = "pub")]
    stream: String,
    #[getset(get = "pub")]
    lang: String,
    #[getset(get = "pub")]
    lang_iso: String,
    #[getset(get = "pub")]
    #[serde(rename = "type")]
    subtitles_type: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Getters)]
#[allow(non_snake_case)]
pub struct MediaInfo {
    #[getset(get = "pub")]
    filename: String,
    #[getset(get = "pub")]
    hoster: String,
    #[getset(get = "pub")]
    link: String,
    #[getset(get = "pub")]
    #[serde(rename = "type")]
    media_type: String,
    #[getset(get = "pub")]
    season: Option<String>,
    #[getset(get = "pub")]
    episode: Option<String>,
    #[getset(get = "pub")]
    year: Option<u16>,
    #[getset(get = "pub")]
    duration: f32,
    #[getset(get = "pub")]
    bitrate: u64,
    #[getset(get = "pub")]
    size: u64,
    #[getset(get = "pub")]
    details: MediaDetails,
    #[getset(get = "pub")]
    poster_path: Option<String>,
    #[getset(get = "pub")]
    audio_image: Option<String>,
    #[getset(get = "pub")]
    backdrop_path: Option<String>,
    #[getset(get = "pub")]
    baseUrl: Option<String>,
    #[getset(get = "pub")]
    availableFormats: Option<HashMap<String, String>>,
    #[getset(get = "pub")]
    availableQualities: Option<HashMap<String,String>>,
    #[getset(get = "pub")]
    modelUrl: Option<String>,
    #[getset(get = "pub")]
    host: Option<String>,
}

/*
    {
        "filename": "string", // Cleaned filename
        "hoster": "string", // File hosted on
        "link": "string", // Original content link
        "type": "string", // "movie" / "show" / "audio"
        "season": "string", // if found, else null
        "episode": "string", // if found, else null
        "year": "string", // if found, else null
        "duration": float, // media duration in seconds
        "bitrate": int, // birate of the media file
        "size": int, // original filesize in bytes
        "details": {
            "video": {
                "und1": { // if available, lang in iso_639 followed by a number ID
                    "stream": "string",
                    "lang": "string", // Language in plain text (ex "English", "French")
                    "lang_iso": "string", // Language in iso_639 (ex fre, eng)
                    "codec": "string", // Codec of the video (ex "h264", "divx")
                    "colorspace": "string", // Colorspace of the video (ex "yuv420p")
                    "width": int, // Width of the video (ex 1980)
                    "height": int // Height of the video (ex 1080)
                }
            },
            "audio": {
                "und1": { // if available, lang in iso_639 followed by a number ID
                    "stream": "string",
                    "lang": "string", // Language in plain text (ex "English", "French")
                    "lang_iso": "string", // Language in iso_639 (ex fre, eng)
                    "codec": "string", // Codec of the audio (ex "aac", "mp3")
                    "sampling": int, // Audio sampling rate
                    "channels": float // Number of channels (ex 2, 5.1, 7.1)
                }
            },
            "subtitles": [
                "und1": { // if available, lang in iso_639 followed by a number ID
                    "stream": "string",
                    "lang": "string", // Language in plain text (ex English, French)
                    "lang_iso": "string", // Language in iso_639 (ex fre, eng)
                    "type": "string" // Format of subtitles (ex "ASS" / "SRT")
                }
            ]
        },
        "poster_path": "string", // URL of the poster image if found / available
        "audio_image": "string", // URL of the music image in HD if found / available
        "backdrop_path": "string" // URL of the backdrop image if found / available
    }
 */


#[allow(non_camel_case_types)]
pub enum ParamsStreaming {
    FROM_DOWNLOAD(Download),
    FROM_UNRESTRICT(Unrestrict),
    FROM_ID(String)
}


