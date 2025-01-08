use {
    chrono::{
        DateTime,
        Utc,
    },
    serde::{
        Deserialize,
        Serialize,
    },
};

pub type CardId = String;
pub type UserId = i64;

// Request
//
// ---
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct NewUser {
    pub card_id: CardId,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct UpdateUser {
    pub id: UserId,
    pub name: Option<String>,
    pub three_letter_name: Option<String>,
    pub identification: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct MatchTransaction {
    pub user_id: UserId,
    pub square_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct SyntheticCharge {
    pub user_id: UserId,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum PosServerRequest {
    RegisterToken(String),
    NewUser(NewUser),
    GetUserByCardId(CardId),
    GetUserBalance(UserId),
    UpdateUser(UpdateUser),
    GetUnmachedOrders,
    MatchOrder(MatchTransaction),
    SyntheticCharge(SyntheticCharge),
    GetTransactions(UserId),
}

// Response
//
// ---
pub type RespRegisterToken = bool;
pub type RespNewUser = ();

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct RespGetUserByCardIdInner {
    pub id: UserId,
    pub card_id: CardId,
    pub name: String,
    pub three_letter_name: String,
    pub identification: String,
}

pub type RespGetUserByCardId = Option<RespGetUserByCardIdInner>;
pub type RespGetUserBalance = i64;
pub type RespUpdateUser = ();

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct RespGetUnmatchedOrdersInner {
    pub id: String,
    pub time: DateTime<Utc>,
    pub coins: u64,
}

pub type RespGetUnmachedOrders = Vec<RespGetUnmatchedOrdersInner>;
pub type RespMatchOrder = ();

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct RespGetTransactionsTransaction {
    pub time: DateTime<Utc>,
    pub source: String,
    pub dest: String,
    pub amount: i64,
}

pub type RespSyntheticCharge = ();
pub type RespGetTransactions = Vec<RespGetTransactionsTransaction>;
