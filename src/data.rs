use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde_with::serde_as]
pub struct SigningData {
    pub sig_bytes: Vec<u8>,
    pub pub_key: Vec<u8>,
}

fn ordered_map<S>(value: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde_with::serde_as]
pub struct UserData {
    pub id: String,
    pub expires: DateTime<Utc>,
    #[serde(serialize_with = "ordered_map")]
    pub features: HashMap<String, String>,
    pub max_users: usize,
    pub key_phrase: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde_with::serde_as]
pub struct License {
    pub user_data: UserData,
    pub signing_data: SigningData,
}

#[derive(std::fmt::Debug, Clone)]
pub enum LicenseError {
    DateFormat(String),
    JSONIncorrect(String),
    FileError(String),
}
