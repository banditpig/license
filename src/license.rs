use std::collections::HashMap;
use std::fs;
use std::fs::File;

use std::io::Write;

use std::path::Path;

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc, MIN_DATE};
use rand::rngs::OsRng;
use schnorrkel::{signing_context, Keypair, PublicKey, Signature};
use uuid::Uuid;

use crate::data::LicenseError::{FileError, JSONIncorrect, SigningProblem, UserDataError};
use crate::data::*;

impl Default for SigningData {
    fn default() -> Self {
        SigningData::new()
    }
}
impl SigningData {
    pub fn new() -> Self {
        Self {
            sig_bytes: Vec::new(),
            pub_key: Vec::new(),
        }
    }
}

impl Default for UserData {
    fn default() -> Self {
        UserData::new()
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
            key_phrase: "".to_string(),
        }
    }
}
impl Default for License {
    fn default() -> Self {
        License::new()
    }
}
impl License {
    pub fn new() -> License {
        License {
            user_data: UserData::new(),
            signing_data: SigningData::new(),
        }
    }

    pub fn with_feature(mut self, key: String, val: String) -> Result<License, LicenseError> {
        if key.is_empty() {
            return Err(UserDataError("Empty key supplied for feature".to_string()));
        }
        if val.is_empty() {
            return Err(UserDataError(
                "Empty value supplied for feature".to_string(),
            ));
        }
        self.user_data.features.insert(key, val);
        Ok(self)
    }

    //Mainly for testing.
    pub fn with_seconds_duration(mut self, secs: i64) -> Result<License, LicenseError> {
        let utc_now = Utc::now();
        let now_plus = utc_now.checked_add_signed(Duration::seconds(secs)).unwrap();

        self.user_data.expires = now_plus;

        Ok(self)
    }
    pub fn with_expiry(mut self, exp: &str) -> Result<License, LicenseError> {
        let naive_date = NaiveDate::parse_from_str(exp, "%Y-%m-%d");
        match naive_date {
            Ok(date) => {
                let from_utc = DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc);
                self.user_data.expires = from_utc;
            }
            Err(msg) => return Err(LicenseError::DateFormat(msg.to_string())),
        }
        Ok(self)
    }
    pub fn with_id(mut self, id: String) -> Result<License, LicenseError> {
        if id.is_empty() {
            return Err(UserDataError("Empty Id supplied".to_string()));
        }
        self.user_data.id = id;
        Ok(self)
    }
    pub fn with_max_users(mut self, max_users: usize) -> Result<License, LicenseError> {
        if max_users == 0 {
            return Err(UserDataError("Must have at least one user".to_string()));
        }
        self.user_data.max_users = max_users;
        Ok(self)
    }
    pub fn with_keyphrase(mut self, keyphrase: String) -> Result<License, LicenseError> {
        if keyphrase.is_empty() {
            return Err(UserDataError("Empty keyphrase supplied".to_string()));
        }
        self.user_data.key_phrase = keyphrase;
        Ok(self)
    }

    pub fn all_from_json(json: &str) -> Result<License, LicenseError> {
        let lic = serde_json::from_str::<License>(json);
        match lic {
            Ok(l) => Ok(l),
            Err(msg) => Err(JSONIncorrect(msg.to_string())),
        }
    }
    pub fn check_license(&self) -> Result<(), LicenseError> {
        match self.verify() {
            Ok(_) => {
                let now = chrono::offset::Utc::now();
                if self.user_data.expires > now {
                    Ok(())
                } else {
                    Err(UserDataError(
                        "License has expired and is out of date.".to_string(),
                    ))
                }
            }
            Err(e) => Err(e),
        }
    }
    pub fn verify(&self) -> Result<(), LicenseError> {
        let byt_arr_sig: &[u8] = &self.signing_data.sig_bytes;
        let byt_arr_pub_key: &[u8] = &self.signing_data.pub_key;

        let signature = match Signature::from_bytes(byt_arr_sig) {
            Ok(s) => s,
            Err(e) => {
                return Err(SigningProblem(
                    "The license file has been tampered with and is invalid.".to_string(),
                ))
            }
        };
        let public_key = match PublicKey::from_bytes(byt_arr_pub_key) {
            Ok(k) => k,
            Err(e) => {
                return Err(SigningProblem(
                    "The license file has been tampered with and is invalid.".to_string(),
                ))
            }
        };

        let context = signing_context(self.user_data.key_phrase.as_bytes());
        let user_data = self.user_data_to_json();
        match public_key.verify(context.bytes(user_data.as_bytes()), &signature) {
            Ok(_) => Ok(()),
            Err(e) => Err(SigningProblem(
                "The license file has been tampered with and is invalid.".to_string(),
            )),
        }
    }
    pub fn build(mut self) -> Result<License, LicenseError> {
        let keypair = Keypair::generate_with(OsRng);

        let user_data = self.user_data_to_json();
        let context = signing_context(self.user_data.key_phrase.as_bytes());
        let signature = keypair.sign(context.bytes(user_data.as_bytes()));

        self.signing_data.sig_bytes = signature.to_bytes().to_vec();
        self.signing_data.pub_key = keypair.public.to_bytes().to_vec();
        Ok(self)
    }
    pub fn all_to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
    fn user_data_to_json(&self) -> String {
        serde_json::to_string(&self.user_data).unwrap()
    }

    /// Saves the License to file.
    ///
    /// # Arguments
    ///
    /// * `path`:
    ///
    /// returns: Result<(), LicenseError>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn save_to_file(&self, path: &str) -> Result<(), LicenseError> {
        let path = Path::new(path);
        let mut file = match File::create(path) {
            Err(e) => return Err(FileError(e.to_string())),
            Ok(file) => file,
        };
        match file.write_all(self.all_to_json().as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(FileError(e.to_string())),
        }
    }
    /// Reads the License from file.
    ///
    /// # Arguments
    ///
    /// * `path`:
    ///
    /// returns: Result<License, LicenseError>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn from_file(path: &str) -> Result<License, LicenseError> {
        match fs::read_to_string(path) {
            Ok(data) => Self::all_from_json(data.as_str()),
            Err(e) => Err(LicenseError::FileError(e.to_string())),
        }
    }
}
