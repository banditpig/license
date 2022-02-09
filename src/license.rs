use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::panic::set_hook;
use std::path::Path;

use chrono::NaiveDate;
use rand::rngs::OsRng;
use schnorrkel::{signing_context, Keypair};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//
//
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde_with::serde_as]
pub struct SigningData {
    sig_bytes: Vec<u8>,
    pub_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde_with::serde_as]
pub struct UserData {
    id: String,
    pub expires: NaiveDate,
    pub features: HashMap<String, String>,
    pub max_users: usize,
    keyphrase: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde_with::serde_as]
pub struct License {
    user_data: UserData,
    signing_data: SigningData,
}

impl SigningData {
    pub fn new() -> Self {
        Self {
            sig_bytes: Vec::new(),
            pub_key: Vec::new(),
        }
    }
}

impl UserData {
    pub fn new() -> Self {
        let date_str = "2000-01-01";
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let guid = Uuid::new_v4();
        Self {
            id: guid.to_string(),
            expires: naive_date,
            features: HashMap::new(),
            max_users: 0,
            keyphrase: "".to_string(),
        }
    }
}

impl License {
    pub fn new() -> License {
        set_hook(Box::new(|info| {
            println!("{:?}", info.to_string());
        }));

        License {
            user_data: UserData::new(),
            signing_data: SigningData::new(),
        }
    }

    pub fn with_feature(mut self, key: String, val: String) -> License {
        self.user_data.features.insert(key, val);
        self
    }

    pub fn with_expiry(mut self, exp: &str) -> License {
        let naive_date = NaiveDate::parse_from_str(exp, "%Y-%m-%d");
        match naive_date {
            Ok(d) => self.user_data.expires = d,
            Err(_) => panic!("Error parsing date {}", exp),
        }

        self
    }
    pub fn with_id(mut self, id: String) -> License {
        self.user_data.id = id;
        self
    }
    pub fn with_max_users(mut self, max_users: usize) -> License {
        self.user_data.max_users = max_users;
        self
    }
    pub fn with_keyphrase(mut self, keyphrase: String) -> License {
        self.user_data.keyphrase = keyphrase;
        self
    }
    pub fn all_to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
    pub fn user_data_to_json(&self) -> String {
        serde_json::to_string(&self.user_data).unwrap()
    }

    pub fn all_from_json(json: &str) -> License {
        serde_json::from_str(json).expect("Unable to make License from supplied json.")
    }
    pub fn sign(mut self) -> License {
        let keypair = Keypair::generate_with(OsRng);
        let message = self.user_data_to_json();
        let context = signing_context(self.user_data.keyphrase.as_bytes());
        let signature = keypair.sign(context.bytes(message.as_bytes()));

        self.signing_data.sig_bytes = signature.to_bytes().to_vec();
        self.signing_data.pub_key = keypair.public.to_bytes().to_vec();
        self
    }
    pub fn save_to_file(&self, path: &str) {
        let path = Path::new(path);
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path.display(), why),
            Ok(file) => file,
        };
        match file.write_all(self.all_to_json().as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
            Ok(_) => println!("successfully wrote to {}", path.display()),
        }
    }
    pub fn from_file(path: &str) -> License {
        let json = fs::read_to_string(path).expect("Problem reading file ");
        Self::all_from_json(json.as_str())
    }
}
