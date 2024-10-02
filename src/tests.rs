#[cfg(test)]
mod tests {
    use crate::{pwd_count, rocket, Pwd};
    use rocket::form::validate::Len;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn validate_default_passwords_returned() {
        let default = pwd_count(6);
        assert_eq!(default.len(), 6)
    }

    #[test]
    fn validate_certain_number_of_passwords_returned() {
        let default = pwd_count(15);
        assert_eq!(default.len(), 15)
    }

    #[test]
    fn validate_max_passwords_returned() {
        let default = pwd_count(1967);
        assert_eq!(default.len(), 31)
    }

    #[test]
    fn validate_no_passwords_returned() {
        let default = pwd_count(0);
        assert_eq!(default.len(), 0)
    }

    #[test]
    fn validate_only_spaces_is_returned() {
        let mut map = std::collections::HashMap::new();
        map.insert("numbers", false);
        map.insert("lowercase_letters", false);
        map.insert("uppercase_letters", false);
        map.insert("symbols", false);
        map.insert("spaces", true);
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/pwd").json(&map).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let passwords: Vec<Pwd> = response.into_json().expect("valid JSON response");
        assert_eq!(
            passwords.first().unwrap().password.replace(" ", "").len(),
            0
        );
    }

    #[test]
    fn validate_number_of_passwords_is_returned() {
        let mut map = std::collections::HashMap::new();
        map.insert("count", 10);
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/pwd").json(&map).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let passwords: Vec<Pwd> = response.into_json().expect("valid JSON response");
        assert_eq!(passwords.len(), 10);
    }
}
