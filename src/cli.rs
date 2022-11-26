use clap::{arg_enum, App, Arg, SubCommand};
use license::data::{License, LicenseError};
use uuid::Uuid;

#[derive(Debug)]
pub struct Args {
    pub features_file: String,
    pub expires: String,
    pub owner_name: String,
    pub owner_email: String,
    pub public_key_file: String,
}
arg_enum! {
    #[derive(Debug)]
    enum Algorithm {
        SHA1,
        SHA256,
        Argon2
    }
}
fn make_license() -> Result<License, LicenseError> {
    let phrase = Uuid::new_v4().to_string();
    License::new()
        .with_days_duration(30)?
        .with_keyphrase(phrase)?
        .build()
}
fn main() {
    let lic = make_license().unwrap();
    let _ = lic.save_to_file("video_stacker.lic");
}
