use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User => name: {}, password: {}",
            self.name, self.password
        )
    }
}

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub password: String,
}

pub struct NewUser {
    pub name: UserName,
    pub password: UserPassword,
}
pub struct UserName(String);
pub struct UserPassword(String);

impl TryInto<NewUser> for FormData {
    type Error = String;

    fn try_into(self) -> Result<NewUser, Self::Error> {
        let vname = validate(self.name).expect("failed to validate name");
        let vpass = validate(self.password).expect("failed to validate pass");

        let name = UserName(vname);
        let password = UserPassword(vpass);

        Ok(NewUser { name, password })
    }
}

fn validate(s: String) -> Result<String, String> {
    let is_empty_or_whitespace = s.trim().is_empty();
    let is_too_long = s.graphemes(true).count() > 256;
    let forbidden_char = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_char = s.chars().any(|c| forbidden_char.contains(&c));

    if is_empty_or_whitespace || is_too_long || contains_forbidden_char {
        Err(format!("{} not a valid username", s))
    } else {
        Ok(s)
    }
}

#[cfg(test)]
mod user_tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    #[test]
    fn create_a_user_ok() {
        let name = "Kenneth".to_string();
        let password = "password".to_string();

        let u = User { name, password };

        assert_eq!("Kenneth".to_string(), u.name);
        assert_eq!("password".to_string(), u.password);
    }

    #[test]
    fn validate_name() {
        let name = "Kenneth".to_string();

        assert_ok!(validate(name));
    }

    #[test]
    fn a_name_longer_than_256_is_rejected() {
        let name = "å".repeat(257);
        assert_err!(validate(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(validate(name));
    }

    #[test]

    fn name_containing_an_invalid_char_is_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(validate(name));
        }
    }
}
