#![allow(dead_code)]

mod data;
mod license;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};

    // use crate::license::License;
    use crate::data::{License, LicenseError};

    #[test]
    fn to_from_file() {
        let lic = make_license().unwrap();
        let _ = lic.save_to_file("lic1.txt");
        let lic2 = License::from_file("lic1.txt").unwrap();
        assert_eq!(lic, lic2);
        let _ = fs::remove_file("lic1.txt");
    }

    #[test]
    fn check_license_expired() {
        let lic = make_early_license().unwrap();
        assert_eq!(lic.check_license(), false);
    }

    #[test]
    fn check_license_not_expired() {
        let lic = make_license().unwrap();
        assert_eq!(lic.check_license(), true);
    }

    #[test]
    fn check_license_expired_from_file() {
        let lic = make_early_license().unwrap();
        let _ = lic.save_to_file("early_lic.txt");
        let lic2 = License::from_file("early_lic.txt").unwrap();
        assert_eq!(lic2.check_license(), false);
        let _ = fs::remove_file("early_lic.txt");
    }

    #[test]
    fn check_license_not_expired_from_file() {
        let lic = make_license().unwrap();
        let _ = lic.save_to_file("lic2.txt");
        let lic2 = License::from_file("lic2.txt").unwrap();
        assert_eq!(lic2.check_license(), true);
        let _ = fs::remove_file("lic2.txt");
    }

    #[test]
    fn verify_from_file() {
        let lic1 = make_license().unwrap();
        let _ = lic1.save_to_file("lic3.txt");

        let lic = License::from_file("lic3.txt").unwrap();
        assert_eq!(lic.verify(), true);
        let _ = fs::remove_file("lic3.txt");
    }

    #[test]
    fn save_to_file_edit_then_verify() {
        let lic = make_license().unwrap();
        let _ = lic.save_to_file("licEdit.txt");

        let file = File::open("licEdit.txt").unwrap();
        let reader = BufReader::new(file);

        let mut writer = &File::create("temp1.txt").unwrap();
        for line in reader.lines() {
            let txt = line.unwrap();

            if !txt.contains("admin") {
                writer.write_all(txt.as_bytes()).unwrap();
            }
        }
        writer.flush().unwrap();

        fs::rename("temp1.txt", "licEdit.txt").unwrap();

        let lic_loaded = License::from_file("licEdit.txt").unwrap();
        assert_eq!(lic_loaded.verify(), false);
        let _ = fs::remove_file("licEdit.txt");
    }

    #[test]
    fn save_to_file_not_pretty_json_then_verify() {
        //This test to check that losing pretty format
        //on the json does not affect the verification.

        let lic = make_license().unwrap();
        let _ = lic.save_to_file("licjson.txt");

        let file = File::open("licjson.txt").unwrap();
        let reader = BufReader::new(file);

        let mut writer = &File::create("temp.txt").unwrap();
        for line in reader.lines() {
            let txt = line.unwrap();
            writer.write_all(txt.as_bytes()).unwrap();
        }
        writer.flush().unwrap();

        fs::rename("temp.txt", "licjson.txt").unwrap();

        let lic_loaded = License::from_file("licjson.txt").unwrap();
        assert_eq!(lic_loaded.verify(), true);
        let _ = fs::remove_file("licjson.txt");
    }

    #[test]
    fn verify() {
        let lic = make_license().unwrap();
        assert_eq!(lic.verify(), true);
    }

    #[test]
    fn to_from_json() {
        let lic = make_license().unwrap();
        let lic_json = lic.all_to_json();
        let lic2 = License::all_from_json(&lic_json).unwrap();
        assert_eq!(lic, lic2);
    }

    fn make_license() -> Result<License, LicenseError> {
        License::new()
            .with_feature("debug".to_string(), "parts1".to_string())?
            .with_feature("emails".to_string(), "email1, email2".to_string())?
            .with_feature("admin".to_string(), "fred, joe".to_string())?
            .with_feature("remote connect".to_string(), "yes".to_string())?
            .with_expiry("2024-02-28")?
            .with_max_users(10)?
            .with_keyphrase("new license being made".to_string())?
            .build()
    }

    fn make_early_license() -> Result<License, LicenseError> {
        License::new()
            .with_feature("debug".to_string(), "parts1".to_string())?
            .with_feature("emails".to_string(), "email1, email2".to_string())?
            .with_feature("admin".to_string(), "fred, joe".to_string())?
            .with_feature("remote connect".to_string(), "yes".to_string())?
            .with_expiry("2018-02-28")?
            .with_max_users(10)?
            .with_keyphrase("new license being made".to_string())?
            .build()
    }
}
