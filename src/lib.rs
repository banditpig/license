#![allow(dead_code)]
mod license;

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::license::License;

    #[test]
    fn build() {
        let date_str = "2024-01-01";
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
        let lic = License::new()
            .with_feature("debug".to_string(), "parts1".to_string())
            .with_feature("emails".to_string(), "email1, email2".to_string())
            .with_feature("admin".to_string(), "fred, joe".to_string())
            .with_feature("remote connect".to_string(), "yes".to_string())
            .with_expiry(naive_date)
            .with_max_users(10)
            .to_json();

        println!("{:?}", lic);
        let lic2 = License::from_json(&lic);
        println!("{:?}", lic2);

    }
}
