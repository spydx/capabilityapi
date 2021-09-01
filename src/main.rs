// Source: https://web.archive.org/web/20180120000131/http://www.zsck.co/writing/capability-based-apis.html

use std::fmt;
use sqlite::{Connection, Value};

struct SQLite {
    db: Connection,
}

#[derive(Debug)]
struct DatabaseError;

struct Save<T>(pub T);
struct Update<T>(pub T);
struct Find<T>(pub T);

trait Capability<Operation> {
    type Data;
    type Error;
    fn perform(&self, _: Operation) -> Result<Self::Data, Self::Error>;
}

struct User {
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

impl Capability<Save<User>> for SQLite {
    type Data = User;
    type Error = DatabaseError;

    fn perform(&self, save_user: Save<User>) -> Result<User, DatabaseError> {
        // insert user to database
        let mut stmt = self
            .db
            .prepare("INSERT INTO users (name, password) VALUES (?1, ?2)")
            .unwrap()
            .into_cursor();

        stmt.bind(&[
            Value::String(save_user.0.name.to_string()),
            Value::String(save_user.0.password.to_string()),
        ])
        .unwrap();

        Ok(save_user.0)
    }
}

impl Capability<Find<User>> for SQLite {
    type Data = User;
    type Error = DatabaseError;

    fn perform(&self, find_user: Find<User>) -> Result<Self::Data, Self::Error> {
        let mut cursor = self
            .db
            .prepare("SELECT name, password FROM users WHERE name = ?")
            .unwrap()
            .into_cursor();
        cursor.bind(&[Value::String(find_user.0.name)]).unwrap();

        let row = cursor.next().unwrap().unwrap();

        let u = User {
            name: row[0].as_string().unwrap().to_string(),
            password: row[1].as_string().unwrap().to_string(),
        };

        Ok(u)
    }
}
impl Capability<Update<User>> for SQLite {
    type Data = User;
    type Error = DatabaseError;

    fn perform(&self, updated_user: Update<User>) -> Result<Self::Data, Self::Error> {
        let mut cursor = self
            .db
            .prepare("UPDATE users SET password = ? WHERE name = ?")
            .unwrap()
            .into_cursor();
        
        cursor.bind(&[
            Value::String(updated_user.0.password.to_string()), 
            Value::String(updated_user.0.name.to_string())])
            .unwrap();
        
        Ok(updated_user.0)
    }
}

fn handle_save_user<DB>(db: &DB, user: User) -> Result<User, DatabaseError>
where
    DB: Capability<Save<User>, Data = User, Error = DatabaseError>,
{
    db.perform(Save(user))
}


fn handle_find_user<DB>(db: &DB, name: String) -> Result<User, DatabaseError>
where
    DB: Capability<Find<User>, Data = User, Error = DatabaseError>,
{
    let user = User { name, password: "".to_string()};
    db.perform(Find(user))
}

fn handle_update_user<DB>(db: &DB, user: User) -> Result<User, DatabaseError> 
where
    DB: Capability<Update<User>, Data = User, Error = DatabaseError>,
{
    db.perform(Update(user))
}

fn main() {
    println!("Hello, world!");

    let connection = sqlite::open(":memory:").unwrap();

    connection
        .execute(
            "CREATE TABLE users (name TEXT,password TEXT);
            INSERT INTO users VALUES ('kenneth', 'password');
            INSERT INTO users VALUES ('boisy', 'woof');",
        )
        .unwrap();

    let db = SQLite { db: connection };

    let user = User {
        name: "Ollie".to_string(),
        password: "pffpff".to_string(),
    };

    let u = handle_save_user(&db, user).unwrap();

    println!("Saved: {}", u);

    let mut boisy = handle_find_user(&db, "boisy".to_string()).unwrap();
    println!("Found: {}", &boisy);

    boisy.password = "WoofWoof".to_string();

    let updated = handle_update_user(&db, boisy).unwrap();
    println!("Updated: {}", updated)

}
