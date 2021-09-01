use crate::model::User;
use sqlite::{Connection, Value};


pub struct SQLite {
     pub db: Connection,
}

#[derive(Debug)]
pub struct DatabaseError;

pub struct Create<T>(pub T);
pub struct Read<T>(pub T);
pub struct Update<T>(pub T);
pub struct Delete<T>(pub T);

pub trait Capability<Operation> {
    type Data;
    type Error;
    fn perform(&self, _: Operation) -> Result<Self::Data, Self::Error>;
}


macro_rules! capability {
    ($name:ident for $type:ty, composing $({$operation:ty, $d:ty, $e:ty}),+) => {
        pub trait $name: $(Capability<$operation, Data = $d, Error = $e>+)+ {}

        impl $name for $type {}
    };
}

capability!(CanCreateUserData for SQLite,
    composing { Create<User>, User, DatabaseError});

capability!(CanReadUserData for SQLite, 
    composing {Read<User>, User, DatabaseError});

capability!(CanUpdateUserData for SQLite, 
    composing { Update<User>, User, DatabaseError});

capability!(CanDeleteUserData for SQLite, 
    composing   { Delete<User>, (), DatabaseError});

capability!(CanReadAndChangeData for SQLite, 
    composing   { Read<User>, User, DatabaseError},
                { Update<User>, User, DatabaseError});


impl Capability<Create<User>> for SQLite {
    type Data = User;
    type Error = DatabaseError;

    fn perform(&self, save_user: Create<User>) -> Result<User, DatabaseError> {
        // insert user to database
        let mut cursor = self
            .db
            .prepare("INSERT INTO users (name, password) VALUES (?1, ?2)")
            .unwrap()
            .into_cursor();

        cursor
            .bind(&[
                Value::String(save_user.0.name.to_string()),
                Value::String(save_user.0.password.to_string()),
            ])
            .unwrap();
        cursor.next().unwrap();

        Ok(save_user.0)
    }
}

impl Capability<Read<User>> for SQLite {
    type Data = User;
    type Error = DatabaseError;

    fn perform(&self, find_user: Read<User>) -> Result<Self::Data, Self::Error> {
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

        cursor
            .bind(&[
                Value::String(updated_user.0.password.to_string()),
                Value::String(updated_user.0.name.to_string()),
            ])
            .unwrap();

        cursor.next().unwrap();

        Ok(updated_user.0)
    }
}

impl Capability<Delete<User>> for SQLite {
    type Data = ();
    type Error = DatabaseError;

    fn perform(&self, user_to_delete: Delete<User>) -> Result<Self::Data, Self::Error> {
        let mut cursor = self.db.prepare("DELETE FROM users WHERE name = ?").unwrap();

        cursor.bind(1, &*user_to_delete.0.name).unwrap();
        cursor.next().unwrap();
        Ok(())
    }
}


pub fn handle_save_user<DB>(db: &DB, user: User) -> Result<User, DatabaseError>
where
    DB: CanCreateUserData,
{
    db.perform(Create(user))
}

pub fn handle_find_user<DB>(db: &DB, name: String) -> Result<User, DatabaseError>
where
    DB: CanReadUserData,
{
    let user = User {
        name,
        password: "".to_string(),
    };
    db.perform(Read(user))
}

pub fn handle_update_user<DB>(db: &DB, user: User) -> Result<User, DatabaseError>
where
    DB: CanUpdateUserData,
{
    db.perform(Update(user))
}

pub fn handle_delete_user<DB>(db: &DB, name: String) -> Result<(), DatabaseError>
where
    DB: CanDeleteUserData,
{
    let u = User {
        name,
        password: "".to_string(),
    };
    db.perform(Delete(u))
}

pub fn display_db_content(con: &SQLite) {
    let mut cursor = con
        .db
        .prepare("SELECT name, password FROM users")
        .unwrap()
        .into_cursor();
    println!("DBContent: ");
    while let Some(row) = cursor.next().unwrap() {
        let u = User {
            name: row[0].as_string().unwrap().to_string(),
            password: row[1].as_string().unwrap().to_string(),
        };
        println!("{}", u)
    }
    println!();

    cursor = con
        .db
        .prepare("SELECT COUNT(*) FROM users")
        .unwrap()
        .into_cursor();

    while let Some(row) = cursor.next().unwrap() {
        println!("Count: {}\n", row[0].as_integer().unwrap());
    }
}