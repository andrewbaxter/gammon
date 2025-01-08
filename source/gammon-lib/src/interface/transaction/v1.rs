use {
    crate::interface::proto::gamestand::v1::AbsClientPaymentId,
    serde::{
        Deserialize,
        Serialize,
    },
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Account {
    // Added via admin commands
    AdminCoins,
    GameCoins(String),
    UserCoins(i64),
    SquareCoins,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Account::AdminCoins => return "Admin".fmt(f),
            Account::GameCoins(game) => return format_args!("Game ({})", game).fmt(f),
            Account::UserCoins(user) => return format_args!("User ({})", user).fmt(f),
            Account::SquareCoins => return "Purchase".fmt(f),
        };
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ExternalId {
    Stand {
        stand: String,
        client_id: AbsClientPaymentId,
    },
    Square(String),
}
