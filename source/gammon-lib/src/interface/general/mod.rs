use {
    serde::{
        Deserialize,
        Serialize,
    },
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct PlayerMeta {
    pub name: String,
    pub three_letter_name: String,
    pub color: [f32; 3],
}
