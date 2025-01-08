use {
    crate::interface::general::PlayerMeta,
    serde::{
        de::DeserializeOwned,
        Deserialize,
        Serialize,
    },
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct GameRelClientPaymentId(pub String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct AbsClientPaymentId {
    pub game_id: String,
    pub id: GameRelClientPaymentId,
}

pub trait RequestTrait: Serialize + DeserializeOwned + Into<Request> {
    type Response: Serialize + DeserializeOwned;
}

// # Request
//
// ## Ask payment
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ReqAskPayment {
    pub client_payment_id: AbsClientPaymentId,
}

impl Into<Request> for ReqAskPayment {
    fn into(self) -> Request {
        return Request::AskPayment(self);
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct Payment {
    pub client_payment_id: AbsClientPaymentId,
    pub player_meta: PlayerMeta,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct RespAskPayment {
    pub payment: Option<Payment>,
}

impl RequestTrait for ReqAskPayment {
    type Response = RespAskPayment;
}

// ## Release payment
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ReqReleasePayment {
    pub client_payment_id: AbsClientPaymentId,
}

impl Into<Request> for ReqReleasePayment {
    fn into(self) -> Request {
        return Request::ReleasePayment(self);
    }
}

impl RequestTrait for ReqReleasePayment {
    type Response = ();
}

// ## Commit/clear
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct ReqCommitClearPayments {
    pub commit_payment_ids: Vec<AbsClientPaymentId>,
}

impl Into<Request> for ReqCommitClearPayments {
    fn into(self) -> Request {
        return Request::CommitClearPayments(self);
    }
}

impl RequestTrait for ReqCommitClearPayments {
    type Response = ();
}

// # Combined
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Request {
    AskPayment(ReqAskPayment),
    ReleasePayment(ReqReleasePayment),
    CommitClearPayments(ReqCommitClearPayments),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct PlayerConfig {
    pub meta: PlayerMeta,
    pub controller: usize,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct GameStartupConfig {
    pub players: Vec<PlayerConfig>,
}
