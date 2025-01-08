use serde::{
    Deserialize,
    Serialize,
};

pub const STAND_GAME_SOCKET: &str = "127.0.0.1:42243";

pub mod v1;

pub use v1 as latest;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ResponseErr(pub String);

impl std::fmt::Display for ResponseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.0.fmt(f);
    }
}

impl std::error::Error for ResponseErr { }

impl<T> Response<T> {
    pub fn into_res(self) -> Result<T, ResponseErr> {
        match self {
            Response::Ok(s) => return Ok(s),
            Response::Err(e) => return Err(ResponseErr(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Response<T> {
    Ok(T),
    Err(String),
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum GameStandReq {
    V1(v1::Request),
}

impl From<v1::Request> for GameStandReq {
    fn from(value: v1::Request) -> Self {
        return Self::V1(value);
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ResError {
    error: String,
}
