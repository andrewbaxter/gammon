use {
    serde::{
        Deserialize,
        Serialize,
    },
};

// Request
//
// ---
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum PosLocalRequest {
    NfcGetId,
    GetIdentity,
}

// Response
//
// ---
pub type RespNfcGetId = Option<String>;
pub type RespGetIdentity = String;
