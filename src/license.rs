use std::collections::HashMap;

use chrono::NaiveDate;
use rand::rngs::OsRng;
use schnorrkel::{signing_context, Keypair};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//
//
#[derive(Debug, Serialize, Deserialize)]
#[serde_with::serde_as]
pub struct License {
    id: String,
    expires: NaiveDate,
    features: HashMap<String, String>,
    max_users: usize,
    keyphrase: String,
    sig_bytes: Vec<u8>,
}

impl License {
    pub fn new() -> License {
        let date_str = "2000-01-01";
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let guid = Uuid::new_v4();
        License {
            id: guid.to_string(),
            expires: naive_date,
            features: HashMap::new(),
            max_users: 0,
            keyphrase: "".to_string(),
            sig_bytes: Vec::new(),
        }
    }
    pub fn from_json(json: &str) -> License {
        let lic: License = serde_json::from_str(json).unwrap();
        lic
    }
    pub fn with_feature(mut self, key: String, val: String) -> License {
        self.features.insert(key, val);
        self
    }

    pub fn with_expiry(mut self, exp: NaiveDate) -> License {
        self.expires = exp;
        self
    }
    pub fn with_id(mut self, id: String) -> License {
        self.id = id;
        self
    }
    pub fn with_max_users(mut self, max_users: usize) -> License {
        self.max_users = max_users;
        self
    }
    pub fn with_keyphrase(mut self, keyphrase: String) -> License {
        self.keyphrase = keyphrase;
        self
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn sign(mut self) -> License {
        let keypair = Keypair::generate_with(OsRng);
        let message = self.to_json();
        let context = signing_context(self.keyphrase.as_bytes());
        let signature = keypair.sign(context.bytes(message.as_bytes()));

        self.sig_bytes = signature.to_bytes().to_vec();
        self
    }
}
