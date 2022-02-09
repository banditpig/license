#![allow(dead_code)]

mod license;

#[cfg(test)]
mod tests {
    use crate::license::License;
    #[test]
    fn to_from_file() {
        let lic = make_license();
        lic.save_to_file("lic.txt");
        let lic2 = lic.from_file("lic.txt");
        assert_eq!(lic, lic2);
    }

    #[test]
    fn to_from_json() {
        let lic = make_license();
        let lic_json = lic.all_to_json();
        let lic2 = lic.all_from_json(&lic_json);

        println!("{:?}", lic2.all_to_json());
        assert_eq!(lic, lic2);
    }

    fn make_license() -> License {
        License::new()
            .with_feature("debug".to_string(), "parts1".to_string())
            .with_feature("emails".to_string(), "email1, email2".to_string())
            .with_feature("admin".to_string(), "fred, joe".to_string())
            .with_feature("remote connect".to_string(), "yes".to_string())
            .with_expiry("2024-02-28")
            .with_max_users(10)
            .with_keyphrase("new license being made".to_string())
            .sign()
    }
}
