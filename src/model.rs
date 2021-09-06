use std::fmt;

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