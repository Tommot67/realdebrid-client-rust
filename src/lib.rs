pub mod data_struct;

use std::collections::HashMap;
use std::thread;
use std::time::{Duration, SystemTime};
use reqwest::{Client, Response, StatusCode};
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::data_struct::download::{Downloads, Download, ParamsDownload};
use crate::data_struct::{RDError, RDOk};
use crate::data_struct::auth::{AuthCredential, AuthDevice, AuthRefresh, AuthToken};
use crate::data_struct::host::{Host, Hosts};
use crate::data_struct::streaming::{MediaInfo, ParamsStreaming, StreamingTranscode};
use crate::data_struct::torrent::{ParamsTorrent, ParamsTorrentFile, ParamsTorrentHost, Torrent, TorrentAdd, TorrentCount, TorrentHost, Torrents};
use crate::data_struct::traffic::{Traffic, TrafficPeriod, Traffics, TrafficsPeriod};
use crate::data_struct::unrestrict::{Unrestrict, UnrestrictCheck};
use crate::data_struct::user::User;

const BASE_URL: &'static str = "https://api.real-debrid.com/rest/1.0/";
const CLIENT_ID: &'static str = "X245A4XAIBGVM";

/// Real-Debrid API Documentation : https://api.real-debrid.com/
#[derive(Default, Debug, Clone)]
pub struct RDClient {
    authorization: String,
    refresh_authorization: Option<AuthRefresh>,
}


pub trait RDTraitAsync {
    async fn auth() -> Result<RDClient, RDError> ;

    async fn refresh_token(&mut self) -> Result<RDOk, RDError> ;

    async fn get_time() -> String ;

    async fn get_time_iso() -> String ;

    async fn disable_access_token(&self) -> Result<(), ()> ;

    async fn get_user(&self) -> Result<User, RDError> ;

    async fn check_unrestrict(&self, link: String, hoster_password: Option<String>) -> Result<UnrestrictCheck, RDError> ;

    async fn unrestrict_link(&self, link: String, hoster_password: Option<String>, remote: Option<bool>) -> Result<Unrestrict, RDError> ;

    async fn unrestrict_folder(&self, link: String) -> Result<Vec<String>, RDError> ;

    async fn unrestrict_decrypt_special_folder(&self) -> Result<Vec<String>, RDError> ;

    async fn unrestrict_decrypt_folder(&self, link: String) -> Result<Vec<String>, RDError> ;

    async fn get_traffic(&self) -> Result<Traffics, RDError> ;

    async fn get_traffic_details(&self, start: Option<String> , end: Option<String> ) -> Result<TrafficsPeriod, RDError> ;

    async fn get_streaming_transcode(&self, streaming: ParamsStreaming) -> Result<StreamingTranscode, RDError> ;

    async fn get_streaming_media_info(&self, streaming: ParamsStreaming) -> Result<MediaInfo, RDError> ;

    async fn get_downloads(&self, offset: Option<u32>, page: Option<u32>, limit: Option<u32>) -> Result<Downloads, RDError> ;

    async fn remove_download(&self, download: ParamsDownload) -> Result<RDOk, RDError> ;

    async fn get_torrents(&self, offset: Option<u32>, page: Option<u32>, limit: Option<u32>, filter: Option<String>) -> Result<Torrents, RDError> ;

    async fn get_torrents_info(&self, torrent: ParamsTorrent) -> Result<Torrent, RDError> ;

    async fn get_torrents_active_count(&self) -> Result<TorrentCount, RDError> ;

    async fn get_torrents_available_hosts(&self) -> Result<Vec<TorrentHost>, RDError> ;

    async fn add_torrent_file(&self, path: String, host: Option<ParamsTorrentHost>) -> Result<TorrentAdd, RDError> ;

    async fn add_torrent_magnet(&self, magnet: String, host: Option<ParamsTorrentHost>) -> Result<TorrentAdd, RDError> ;

    async fn select_torrent_file(&self, torrent: ParamsTorrent, files: ParamsTorrentFile) -> Result<(),RDError> ;

    async fn remove_torrent(&self, torrent: ParamsTorrent) -> Result<RDOk, RDError> ;

    async fn get_host() -> Hosts ;

    async fn get_host_regex() -> Vec<String> ;

    async fn get_host_regex_folder() -> Vec<String> ;

    async fn get_host_domains() -> Vec<String> ;

    async fn get_host_with_status(&self) -> Result<Hosts, RDError> ;

}

pub trait RDTrait {
    fn new(api_key: String) -> RDClient;
    fn auth_valid(&self) -> Result<bool, RDError>;
    fn change_api_key(&mut self, api_key: String);
    fn create_link(other_part: &str, params: Option<&str>) -> String;
    fn create_auth(api_key: String) -> String;
}

impl RDTrait for RDClient {
    
    /// Create new RDClient with api key.
    fn new(api_key: String) -> RDClient {
        RDClient { authorization: Self::create_auth(api_key), refresh_authorization: None }
    }

    /// Check if oauth2 is valid or if is necessary to refresh.
    fn auth_valid(&self) -> Result<bool, RDError> {
        if self.refresh_authorization.is_none() {
            Err(RDError::NOT_OAUTH2)
        }
        else {
            let auth_refresh = self.refresh_authorization.clone().unwrap();
            Ok(auth_refresh.auth_time.elapsed().unwrap().gt(&Duration::from_secs(auth_refresh.expires_in)))
        }
    }

    /// Change current api key with new api key.
    fn change_api_key(&mut self, api_key: String) {
        self.authorization = Self::create_auth(api_key);
    }

    /// Create link with BASE_URL and other part add options.
    fn create_link(other_part: &str, params: Option<&str>) -> String {
        format!("{}{}?{}", BASE_URL , other_part, params.unwrap_or(""))
    }

    /// Create header format for auth with api key.
    fn create_auth(api_key: String) -> String {
        format!("Bearer {}", api_key)
    }
    
}

impl RDTraitAsync for RDClient {

    /// Create new RDClient with oauth2.
    async fn auth() -> Result<RDClient, RDError> {

        let client = Client::new();

        let response1 = client.get(format!("https://api.real-debrid.com/oauth/v2/device/code?client_id={}&new_credentials=oui", CLIENT_ID)).send().await.unwrap();
        if response1.status() != StatusCode::OK {
            return Err(RDError::AUTH_FAILED);
        }

        let result1 = response1.json::<AuthDevice>().await.unwrap();

        println!("Verifie device on link {} with code {}", result1.verification_url, result1.user_code);

        let mut pass = 0;
        let mut response2: Response;
        loop {
            if pass * result1.interval > result1.expires_in {
                return Err(RDError::AUTH_FAILED);
            }
            pass += 1;

            response2 = client.get(format!("https://api.real-debrid.com/oauth/v2/device/credentials?client_id={}&code={}", CLIENT_ID, result1.device_code.clone())).send().await.unwrap();
            if response2.status() != StatusCode::OK {
                thread::sleep(Duration::from_secs(result1.interval));
            }
            else {
                break;
            }
        }

        let result2 = response2.json::<AuthCredential>().await.unwrap();

        let mut params = HashMap::new();
        params.insert("client_id", result2.client_id.to_string());
        params.insert("client_secret", result2.client_secret.to_string());
        params.insert("code", result1.device_code);
        params.insert("grant_type", "http://oauth.net/grant_type/device/1.0".to_string());

        let response3 = client.post("https://api.real-debrid.com/oauth/v2/token").form(&params).send().await.unwrap();
        if response3.status() != StatusCode::OK {
            return Err(RDError::AUTH_FAILED);
        }

        let result3 = response3.json::<AuthToken>().await.unwrap();

        let auth_refresh = AuthRefresh { client_id: result2.client_id , client_secret: result2.client_secret, refresh_token: result3.refresh_token, auth_time: SystemTime::now(), expires_in: result3.expires_in};

        Ok(RDClient { authorization: format!("{} {}", result3.token_type, result3.access_token), refresh_authorization: Some(auth_refresh) })
    }

    /// Refresh RDClient when use oauth2.
    async fn refresh_token(&mut self) -> Result<RDOk, RDError> {
        if self.refresh_authorization.is_none() {
            Err(RDError::NOT_REFRESH_TOKEN)
        }
        else {
            let mut auth_refresh = self.refresh_authorization.clone().unwrap();
            let mut params = HashMap::new();
            params.insert("client_id", auth_refresh.client_id.to_string());
            params.insert("client_secret", auth_refresh.client_secret.to_string());
            params.insert("code", auth_refresh.refresh_token);
            params.insert("grant_type", "http://oauth.net/grant_type/device/1.0".to_string());

            let response = Client::new().post("https://api.real-debrid.com/oauth/v2/token").form(&params).send().await.unwrap();
            if response.status() != StatusCode::OK {
                self.refresh_authorization = None;
                return Err(RDError::REFRESH_FAILED);
            }

            let result = response.json::<AuthToken>().await.unwrap();

            auth_refresh.refresh_token = result.refresh_token;
            auth_refresh.auth_time = SystemTime::now();
            auth_refresh.expires_in = result.expires_in;

            self.refresh_authorization = Some(auth_refresh);

            Ok(RDOk::AUTH_REFRESH)

        }
    }

    /// Get server time.
    async fn get_time() -> String {
        Client::new().get(Self::create_link("time", None)).send().await.unwrap().text().await.unwrap()
    }

    /// Get server time in ISO.
    async fn get_time_iso() -> String {
        Client::new().get(Self::create_link("time/iso", None)).send().await.unwrap().text().await.unwrap()
    }

    /// Disable current access token
    async fn disable_access_token(&self) -> Result<(), ()> {

        if Client::new().get(Self::create_link("disable_access_token", None)).header("Authorization", self.authorization.to_string()).send().await.unwrap().status() == StatusCode::OK {
            Ok(())
        }
        else {
            Err(())
        }

    }

    /// Get current user info.
    async fn get_user(&self) -> Result<User, RDError> {

        let response = Client::new().get(Self::create_link("user", None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else {
            Ok(response.json::<User>().await.unwrap())
        }

    }

    /// Check a link.
    async fn check_unrestrict(&self, link: String, hoster_password: Option<String>) -> Result<UnrestrictCheck, RDError> {

        let mut params = HashMap::new();
        params.insert("link", link);
        if hoster_password.is_some() {
            params.insert("password", hoster_password.unwrap());
        }

        let response = Client::new().post(Self::create_link("unrestrict/check", None)).header("Authorization", self.authorization.to_string()).form(&params).send().await.unwrap();

        if response.status() == StatusCode::SERVICE_UNAVAILABLE {
            Err(RDError::FILE_UNAVAILABLE)
        }
        else if response.status() == StatusCode::BAD_REQUEST {
            Err(RDError::BAD_REQUEST)
        }
        else {
            Ok(response.json::<UnrestrictCheck>().await.unwrap())
        }

    }

    /// Unrestrict a link.
    async fn unrestrict_link(&self, link: String, hoster_password: Option<String>, remote: Option<bool>) -> Result<Unrestrict, RDError> {
        let mut params = HashMap::new();
        params.insert("link", link.clone());

        if hoster_password.is_some() {
            params.insert("password", hoster_password.unwrap());
        }
        if remote.is_some() {
            params.insert("remote", remote.unwrap().to_string());
        }

        let response = Client::new().post(Self::create_link("unrestrict/link", None)).header("Authorization", self.authorization.to_string()).form(&params).send().await.unwrap();

        if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else {
            Ok(response.json::<Unrestrict>().await.unwrap())
        }

    }

    /// Unrestrict a folder link.
    async fn unrestrict_folder(&self, link: String) -> Result<Vec<String>, RDError> {
        let mut params = HashMap::new();
        params.insert("link", link);

        let response = Client::new().post(Self::create_link("unrestrict/folder", None)).header("Authorization", self.authorization.to_string()).form(&params).send().await.unwrap();

        if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else {
            Ok(response.json::<Vec<String>>().await.unwrap())
        }

    }

    /// Decrypt container file.
    async fn unrestrict_decrypt_special_folder(&self) -> Result<Vec<String>, RDError> {

        let response = Client::new().put(Self::create_link("unrestrict/containerFile", None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::BAD_REQUEST {
            Err(RDError::BAD_REQUEST)
        }
        else if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::NOT_PREMIUM)
        }
        else if response.status() == StatusCode::SERVICE_UNAVAILABLE {
            Err(RDError::SERVICE_UNAVAILABLE)
        }
        else {
            Ok(response.json::<Vec<String>>().await.unwrap())
        }

    }

    /// Decrypt container file from link.
    async fn unrestrict_decrypt_folder(&self, link: String) -> Result<Vec<String>, RDError> {
        let mut params = HashMap::new();
        params.insert("link", link);

        let response = Client::new().post(Self::create_link("unrestrict/containerLink", None)).header("Authorization", self.authorization.to_string()).form(&params).send().await.unwrap();

        if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else {
            Ok(response.json::<Vec<String>>().await.unwrap())
        }

    }

    /// Traffic informations for limited hosters.
    async fn get_traffic(&self) -> Result<Traffics, RDError> {

        let response = Client::new().get(Self::create_link("traffic", None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else {
            let mut traffic: Traffics = Traffics::default();
            traffic.result = response.json::<HashMap<String, Traffic>>().await.unwrap();
            Ok(traffic)
        }

    }

    /// Traffic details on used hosters.
    /// start : Date(YYYY-MM-DD) and end : Date(YYYY-MM-DD).
    async fn get_traffic_details(&self, start: Option<String> , end: Option<String> ) -> Result<TrafficsPeriod, RDError> {
        let mut params: String = String::new();
        if start.is_some() {
            params.push_str(format!("start={}&", start.unwrap()).as_str());
        }
        if end.is_some() {
            params.push_str(format!("end={}&", end.unwrap()).as_str());
        }

        let response = Client::new().get(Self::create_link("traffic/details", Some(params.as_str()))).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else {
            let mut traffic = TrafficsPeriod::default();
            traffic.result = response.json::<HashMap<String, TrafficPeriod>>().await.unwrap();
            Ok(traffic)
        }

    }

    /// Get transcoding links for given file.
    #[allow(unused_assignments)]
    async fn get_streaming_transcode(&self, streaming: ParamsStreaming) -> Result<StreamingTranscode, RDError> {
        let mut id_streaming = String::new();
        match streaming {
            ParamsStreaming::FROM_DOWNLOAD(d) => id_streaming = d.id().to_string(),
            ParamsStreaming::FROM_UNRESTRICT(d) =>  id_streaming = d.id().to_string(),
            ParamsStreaming::FROM_ID(d) => id_streaming = d,
        };

        let response = Client::new().get(Self::create_link(format!("streaming/transcode/{}", id_streaming).as_str(), None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else {
            Ok(response.json::<StreamingTranscode>().await.unwrap())
        }
    }

    /// Get media informations for given file.
    #[allow(unused_assignments)]
    async fn get_streaming_media_info(&self, streaming: ParamsStreaming) -> Result<MediaInfo, RDError> {
        let mut id_streaming = String::new();
        match streaming {
            ParamsStreaming::FROM_DOWNLOAD(d) => id_streaming = d.id().to_string(),
            ParamsStreaming::FROM_UNRESTRICT(d) => id_streaming = d.id().to_string(),
            ParamsStreaming::FROM_ID(d) => id_streaming = d,
        };

        let response = Client::new().get(Self::create_link(format!("streaming/mediaInfos/{}", id_streaming).as_str(), None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::SERVICE_UNAVAILABLE {
            Err(RDError::PROBLEM_FINDING_METADATA)
        }
        else {
            Ok(response.json::<MediaInfo>().await.unwrap())
        }
    }

    /// Get user downloads list.
    async fn get_downloads(&self, offset: Option<u32>, page: Option<u32>, limit: Option<u32>) -> Result<Downloads, RDError> {

        let mut params: String = String::new();
        if offset.is_some() {
            params.push_str(format!("offset={}&", offset.unwrap()).as_str());
        }
        if page.is_some() {
            params.push_str(format!("page={}&", page.unwrap()).as_str());
        }
        if limit.is_some() {
            params.push_str(format!("limit={}", limit.unwrap()).as_str());
        }


        let response = Client::new().get(Self::create_link("downloads", Some(params.as_str()))).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::NO_CONTENT {
            Err(RDError::NO_CONTENT)
        }
        else {
            let mut downloads = Downloads::default();
            downloads.total_count = response.headers().get("x-total-count").unwrap().to_str().unwrap().parse::<u64>().unwrap();
            downloads.result = response.json::<Vec<Download>>().await.unwrap();
            Ok(downloads)
        }

    }

    /// Delete a link from downloads list.
    /// Use Params with id or Download.
    #[allow(unused_assignments)]
    async fn remove_download(&self, download: ParamsDownload) -> Result<RDOk, RDError> {
        let mut id_remove = String::new();
        match download {
            ParamsDownload::FROM_STRUCT(d) => id_remove = d.id().to_string(),
            ParamsDownload::FROM_ID(d) => id_remove = d,
        }

        let response = Client::new().delete(Self::create_link(format!("downloads/delete/{}",id_remove).as_str(), None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::NOT_FOUND {
            Err(RDError::UNKNOWN_RESSOURCE)
        }
        else if response.status() == StatusCode::NO_CONTENT  {
            Ok(RDOk::REMOVED_SUCCESS)
        }
        else {
            println!("{:?}", response.status());
            Err(RDError::UNDEFINED)
        }

    }

    /// Get user torrents list.
    async fn get_torrents(&self, offset: Option<u32>, page: Option<u32>, limit: Option<u32>, filter: Option<String>) -> Result<Torrents, RDError> {

        let mut params: String = String::new();
        if offset.is_some() {
            params.push_str(format!("offset={}&", offset.unwrap()).as_str());
        }
        if page.is_some() {
            params.push_str(format!("page={}&", page.unwrap()).as_str());
        }
        if limit.is_some() {
            params.push_str(format!("limit={}", limit.unwrap()).as_str());
        }
        if filter.is_some() {
            params.push_str(format!("filter={}", filter.unwrap()).as_str());
        }

        let response = Client::new().get(Self::create_link("torrents", Some(params.as_str()))).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::NO_CONTENT {
            Err(RDError::NO_CONTENT)
        }
        else {
            let mut torrents = Torrents::default();
            torrents.total_count = response.headers().get("x-total-count").unwrap().to_str().unwrap().parse::<u64>().unwrap();
            torrents.result = response.json::<Vec<Torrent>>().await.unwrap();
            Ok(torrents)
        }

    }

    /// Get infos on torrent.
    #[allow(unused_assignments)]
    async fn get_torrents_info(&self, torrent: ParamsTorrent) -> Result<Torrent, RDError> {

        let mut id_torrent = String::new();

        match torrent {
            ParamsTorrent::FROM_STRUCT(d) => id_torrent = d.id().to_string(),
            ParamsTorrent::FROM_ADD(d) => id_torrent = d.id().to_string(),
            ParamsTorrent::FROM_ID(d) => id_torrent = d,
        }

        let response = Client::new().get(Self::create_link(format!("torrents/info/{}",id_torrent).as_str(), None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::NO_CONTENT {
            Err(RDError::NO_CONTENT)
        }
        else {
            Ok(response.json::<Torrent>().await.unwrap())
        }

    }

    /// Get currently active torrents number.
    async fn get_torrents_active_count(&self) -> Result<TorrentCount, RDError> {
        let response = Client::new().get(Self::create_link("torrents/activeCount", None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else {
            Ok(response.json::<TorrentCount>().await.unwrap())
        }
    }

    /// Get available hosts.
    async fn get_torrents_available_hosts(&self) -> Result<Vec<TorrentHost>, RDError> {
        let response = Client::new().get(Self::create_link("torrents/availableHosts", None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else {
            Ok(response.json::<Vec<TorrentHost>>().await.unwrap())
        }
    }

    /// Add torrent file.
    async fn add_torrent_file(&self, path: String, host: Option<ParamsTorrentHost>) -> Result<TorrentAdd, RDError> {
        let mut params: String = String::new();
        if host.is_some() {
            match host.unwrap() {
                ParamsTorrentHost::FROM_STRUCT(d) => params.push_str(d.host().as_str()),
                ParamsTorrentHost::FROM_HOST(d) => params.push_str(d.as_str()),
            }
        }

        if !fs::try_exists(path.clone()).await.unwrap() {
            return Err(RDError::PATH_NOT_RIGHT);
        }

        let mut file = File::open(path).await.unwrap();
        // Read the entire file into a buffer
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await.unwrap();

        let response = Client::new().put(Self::create_link("torrents/addTorrent", Some(params.as_str()))).header("Authorization", self.authorization.to_string()).body(buffer).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::NOT_PREMIUM)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::BAD_REQUEST {
            Err(RDError::BAD_REQUEST)
        }
        else if response.status() == StatusCode::SERVICE_UNAVAILABLE {
            Err(RDError::SERVICE_UNAVAILABLE)
        }
        else if response.status() == StatusCode::CREATED {
            Ok(response.json::<TorrentAdd>().await.unwrap())
        }
        else {
            Err(RDError::UNDEFINED)
        }

    }

    /// Add magnet link.
    async fn add_torrent_magnet(&self, magnet: String, host: Option<ParamsTorrentHost>) -> Result<TorrentAdd, RDError> {

        let mut params = HashMap::new();
        params.insert("magnet", magnet);

        if host.is_some() {
            match host.unwrap() {
                ParamsTorrentHost::FROM_STRUCT(d) => params.insert("host", d.host().to_string()),
                ParamsTorrentHost::FROM_HOST(d) => params.insert("host", d),
            };
        }

        println!("{:?}", params);

        let response = Client::new().post(Self::create_link("torrents/addMagnet", None)).header("Authorization", self.authorization.to_string()).form(&params).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::NOT_PREMIUM)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::BAD_REQUEST {
            Err(RDError::BAD_REQUEST)
        }
        else if response.status() == StatusCode::SERVICE_UNAVAILABLE {
            Err(RDError::SERVICE_UNAVAILABLE)
        }
        else if response.status() == StatusCode::CREATED {
            Ok(response.json::<TorrentAdd>().await.unwrap())
        }
        else {
            Err(RDError::UNDEFINED)
        }

    }

    /// Select files of a torrent.
    #[allow(unused_assignments)]
    async fn select_torrent_file(&self, torrent: ParamsTorrent, files: ParamsTorrentFile) -> Result<(),RDError> {
        let mut id_torrent = String::new();
        match torrent {
            ParamsTorrent::FROM_STRUCT(d) => id_torrent = d.id().to_string(),
            ParamsTorrent::FROM_ADD(d) => id_torrent = d.id().to_string(),
            ParamsTorrent::FROM_ID(d) => id_torrent = d,
        };

        let mut params = HashMap::new();
        match files {
            ParamsTorrentFile::FROM_ALL => params.insert("files", "all".to_string()),
            ParamsTorrentFile::FROM_STRUCTS(d) => {
                let ids =  d.iter().map(|ft| ft.id().to_string()).collect::<Vec<String>>();
                params.insert("files", ids.join(","))
            },
            ParamsTorrentFile::FROM_IDS(d) => params.insert("files", d.join(",")),
        };

        let response = Client::new().post(Self::create_link(format!("torrents/selectFiles/{}", id_torrent).as_str(), None)).header("Authorization", self.authorization.to_string()).form(&params).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::NOT_PREMIUM)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::BAD_REQUEST {
            Err(RDError::BAD_REQUEST)
        }
        else if response.status() == StatusCode::NOT_FOUND {
            Err(RDError::UNKNOWN_RESSOURCE)
        }
        else if response.status() == StatusCode::ACCEPTED {
            Err(RDError::ACTION_ALREADY_DONE)
        }
        else if response.status() == StatusCode::NO_CONTENT {
            Ok(())
        }
        else {
            Err(RDError::UNDEFINED)
        }

    }

    /// Delete a torrent from torrents list.
    #[allow(unused_assignments)]
    async fn remove_torrent(&self, torrent: ParamsTorrent) -> Result<RDOk, RDError> {
        let mut id_remove = String::new();
        match torrent {
            ParamsTorrent::FROM_STRUCT(d) => id_remove = d.id().to_string(),
            ParamsTorrent::FROM_ADD(d) => id_remove = d.id().to_string(),
            ParamsTorrent::FROM_ID(d) => id_remove = d,
        }

        let response = Client::new().delete(Self::create_link(format!("torrent/delete/{}",id_remove).as_str(), None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::FORBIDDEN {
            Err(RDError::PERMISSION_DENIED)
        }
        else if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else if response.status() == StatusCode::NOT_FOUND {
            Err(RDError::UNKNOWN_RESSOURCE)
        }
        else if response.status() == StatusCode::NO_CONTENT  {
            Ok(RDOk::REMOVED_SUCCESS)
        }
        else {
            println!("{:?}", response.status());
            Err(RDError::UNDEFINED)
        }

    }

    /// Get supported hosts.
    async fn get_host() -> Hosts {
        let response = Client::new().get(Self::create_link("hosts", None)).send().await.unwrap();
        let mut hosts = Hosts::default();
        hosts.result = response.json::<HashMap<String, Host>>().await.unwrap();
        hosts.with_status = false;
        hosts
    }

    /// Get all supported regex.
    async fn get_host_regex() -> Vec<String> {
        Client::new().get(Self::create_link("hosts/regex", None)).send().await.unwrap().json::<Vec<String>>().await.unwrap()
    }

    /// Get all supported regex for folder links.
    async fn get_host_regex_folder() -> Vec<String> {
        Client::new().get(Self::create_link("hosts/regexFolder", None)).send().await.unwrap().json::<Vec<String>>().await.unwrap()
    }

    /// Get all supported domains.
    async fn get_host_domains() -> Vec<String> {
        Client::new().get(Self::create_link("hosts/domains", None)).send().await.unwrap().json::<Vec<String>>().await.unwrap()
    }

    /// Get status of hosters.
    async fn get_host_with_status(&self) -> Result<Hosts, RDError> {
        let response = Client::new().get(Self::create_link("hosts/status", None)).header("Authorization", self.authorization.to_string()).send().await.unwrap();

        if response.status() == StatusCode::UNAUTHORIZED {
            Err(RDError::BAD_TOKEN)
        }
        else {
            let mut hosts = Hosts::default();
            hosts.result = response.json::<HashMap<String, Host>>().await.unwrap();
            hosts.with_status = true;
            Ok(hosts)
        }

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {

        //code here

    }
}
