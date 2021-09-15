use serde::{Deserialize, Serialize};
use std::fmt;

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

#[cfg(test)]
mod user_tests {
    use super::*;

    #[test]
    fn create_a_user_ok() {
        let name = "Kenneth".to_string();
        let password = "password".to_string();

        let u = User { name, password };

        assert_eq!("Kenneth".to_string(), u.name);
        assert_eq!("password".to_string(), u.password);
    }
}