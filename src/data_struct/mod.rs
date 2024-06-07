pub mod user;
pub mod traffic;
pub mod download;
pub mod host;
pub mod torrent;
pub mod streaming;
pub mod unrestrict;
pub(crate) mod auth;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum RDError {
    UNDEFINED,
    BAD_TOKEN,
    PERMISSION_DENIED,
    NO_CONTENT,
    UNKNOWN_RESSOURCE,
    NOT_PREMIUM,
    BAD_REQUEST,
    SERVICE_UNAVAILABLE,
    PATH_NOT_RIGHT,
    ACTION_ALREADY_DONE,
    PROBLEM_FINDING_METADATA,
    FILE_UNAVAILABLE,
    AUTH_FAILED,
    REFRESH_FAILED,
    NOT_REFRESH_TOKEN,
    NOT_OAUTH2,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum RDOk {
    UNDEFINED,
    REMOVED_SUCCESS,
    ADDED_SUCCESS,
    AUTH_REFRESH,
}