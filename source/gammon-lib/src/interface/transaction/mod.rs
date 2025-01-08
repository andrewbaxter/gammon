use {
    std::borrow::Cow,
    good_ormning_runtime::sqlite::{
        GoodOrmningCustomString,
    },
    serde::{
        Serialize,
        Deserialize,
    },
};

pub mod v1;

pub use v1 as latest;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Account {
    V1(v1::Account),
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Account::V1(v) => return v.fmt(f),
        }
    }
}

impl GoodOrmningCustomString<Account> for Account {
    fn to_sql<'a>(value: &'a Account) -> Cow<'a, str> {
        Cow::Owned(serde_json::to_string(value).unwrap())
    }

    fn from_sql(value: String) -> Result<Account, String> {
        serde_json::from_str(&value).map_err(|e| e.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct Location(pub String);

impl GoodOrmningCustomString<Location> for Location {
    fn to_sql<'a>(value: &'a Location) -> Cow<'a, str> {
        Cow::Borrowed(&value.0)
    }

    fn from_sql(value: String) -> Result<Location, String> {
        return Ok(Location(value));
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ExternalId {
    V1(v1::ExternalId),
}

impl GoodOrmningCustomString<ExternalId> for ExternalId {
    fn to_sql<'a>(value: &'a ExternalId) -> Cow<'a, str> {
        Cow::Owned(serde_json::to_string(value).unwrap())
    }

    fn from_sql(value: String) -> Result<ExternalId, String> {
        serde_json::from_str(&value).map_err(|e| e.to_string())
    }
}
