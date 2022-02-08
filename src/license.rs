use std::collections::HashMap;
use chrono::{ NaiveDate};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Result;
//
//
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct License{
    id: String,
    expires: NaiveDate,
    features: HashMap<String, String>,
    max_users: usize,

}
type Signature = String;

impl License{
    pub fn new() -> License{
        let date_str = "2000-01-01";
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let guid = Uuid::new_v4();
       License {
            id: guid.to_string(),
            expires: naive_date,
            features: HashMap::new(),
            max_users: 0

        }
    }
    pub fn from_json(json: &str) -> License {
       let lic : License =  serde_json::from_str(json).unwrap();
        lic
    }
    pub fn with_feature(mut self, key:String, val: String) -> License {
        self.features.insert(key, val);
        self
    }

    pub fn with_expiry(mut self, exp: NaiveDate ) -> License {
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
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()

    }

}