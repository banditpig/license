use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use chrono::{DateTime, NaiveDate, Utc, MIN_DATE};
use rand::rngs::OsRng;
use schnorrkel::{signing_context, Keypair, PublicKey, Signature};
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

//use std::panic::set_hook;

fn ordered_map<S>(value: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

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
    pub expires: DateTime<Utc>,
    #[serde(serialize_with = "ordered_map")]
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
        let guid = Uuid::new_v4();
        Self {
            id: guid.to_string(),
            expires: MIN_DATE.and_hms(0, 0, 0),
            features: HashMap::new(),
            max_users: 0,
            keyphrase: "".to_string(),
        }
    }
}

impl License {
    pub fn new() -> License {
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
            Ok(date) => {
                let from_utc = DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc);
                self.user_data.expires = from_utc;
            }
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
        serde_json::to_string(&self).unwrap()
    }
    pub fn user_data_to_json(&self) -> String {
        serde_json::to_string(&self.user_data).unwrap()
    }

    pub fn all_from_json(json: &str) -> License {
        serde_json::from_str(json).expect("Unable to make License from supplied json.")
    }
    pub fn check_license(&self) -> bool {
        //verify
        //then check date not expired
        if !self.verify() {
            return false;
        }
        let now = chrono::offset::Utc::now();
        self.user_data.expires > now
    }
    pub fn verify(&self) -> bool {
        let byt_arr_sig: &[u8] = &self.signing_data.sig_bytes;
        let signature = Signature::from_bytes(byt_arr_sig).unwrap();

        let byt_arr_pub_key: &[u8] = &self.signing_data.pub_key;

        let public_key = PublicKey::from_bytes(byt_arr_pub_key).unwrap();
        let context = signing_context(self.user_data.keyphrase.as_bytes());
        let user_data = self.user_data_to_json();
        let res = public_key.verify(context.bytes(user_data.as_bytes()), &signature);
        res.is_ok()
    }
    pub fn sign(mut self) -> License {
        let keypair = Keypair::generate_with(OsRng);
        let user_data = self.user_data_to_json();
        let context = signing_context(self.user_data.keyphrase.as_bytes());
        let signature = keypair.sign(context.bytes(user_data.as_bytes()));

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
        if let Err(why) = file.write_all(self.all_to_json().as_bytes()) {
            panic!("couldn't write to {}: {}", path.display(), why)
        }
    }
    pub fn from_file(path: &str) -> License {
        let data = fs::read_to_string(path).expect("Problem reading file ");
        Self::all_from_json(data.as_str())
    }
}
