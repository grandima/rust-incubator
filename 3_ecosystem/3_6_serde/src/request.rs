use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(rename = "type")]
    pub type_field: String,
    pub stream: Stream,
    pub gifts: Vec<Gift>,
    pub debug: Debug,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "is_private")]
    pub is_private: bool,
    pub settings: i64,
    #[serde(rename = "shard_url")]
    pub shard_url: String,
    #[serde(rename = "public_tariff")]
    pub public_tariff: PublicTariff,
    #[serde(rename = "private_tariff")]
    pub private_tariff: PrivateTariff,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTariff {
    pub id: i64,
    pub price: i64,
    pub duration: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateTariff {
    #[serde(rename = "client_price")]
    pub client_price: i64,
    pub duration: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gift {
    pub id: i64,
    pub price: i64,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Debug {
    pub duration: String,
    pub at: String,
}

