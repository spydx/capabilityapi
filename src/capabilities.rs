use crate::model::User;
use sqlx::{Acquire, SqliteConnection};
use async_trait::async_trait;

pub struct SQLite {
    pub db: SqliteConnection,
}

#[derive(Debug)]
pub struct DatabaseError;

pub struct Create<T>(pub T);
pub struct Read<T>(pub T);
pub struct Update<T>(pub T);
pub struct Delete<T>(pub T);

#[async_trait]
pub trait Capability<Operation> {
    type Data;
    type Error;
    async fn perform(&self, _: Operation) -> Result<Self::Data, Self::Error>;
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

#[async_trait]
impl Capability<Create<User>> for SQLite {
    type Data = User;
    type Error = DatabaseError;

    async fn perform(&self, save_user: Create<User>) -> Result<User, DatabaseError> {
        let mut access = self.db.acquire().await.expect("Unable to get db");

        let r = sqlx::query!(r#"INSERT INTO users (name, password) VALUES (?1, ?2)"#, 
            save_user.0.name.to_string(), 
            save_user.0.password.to_string())
            .excute(&mut access)
            .await
            .map_err(|e| e);

        Ok(save_user.0)
    }
}

#[async_trait]
impl Capability<Read<User>> for SQLite {
    type Data = User;
    type Error = DatabaseError;

    async fn perform(&self, find_user: Read<User>) -> Result<Self::Data, Self::Error> {
        
        let row = sqlx::query!(r#"SELECT name, password FROM users WHERE name = ?"#,
                find_user.0.name.to_string())
                .fetch_one(self.db)
                .await
                .map_err(|e| e);        
        let u = User {
            name: row[0].as_string().unwrap().to_string(),
            password: row[1].as_string().unwrap().to_string(),
        };

        Ok(u)
    }
}

#[async_trait]
impl Capability<Update<User>> for SQLite {
    type Data = User;
    type Error = DatabaseError;

    async fn perform(&self, updated_user: Update<User>) -> Result<Self::Data, Self::Error> {
        let mut access = self.db.acquire().await.expect("Unable to get db");

        let r = sqlx::query!(r#"UPDATE users SET pasword = ?1 WHERE name = ?"#,
                updated_user.0.password.to_string(),
                updated_user.0.name.to_string())
            .execute(&mut access)
            .await
            .map_err(|e| e);

        Ok(updated_user.0)
    }
}

#[async_trait]
impl Capability<Delete<User>> for SQLite {
    type Data = ();
    type Error = DatabaseError;

    async fn perform(&self, user_to_delete: Delete<User>) -> Result<Self::Data, Self::Error> {
        let mut access = self.db.acquire().await.expect("Unable to get db");

        let r = sqlx::query!(r#"DELETE FROM users WHERE name = ?"#,
            user_to_delete.0.name.to_string())
            .execute(&mut access)
            .await
            .map_err(|e| e);

        Ok(())
    }
}

pub async fn handle_save_user<DB>(db: &DB, user: User) -> Result<User, DatabaseError>
where
    DB: CanCreateUserData,
{
    db.perform(Create(user)).await
}

pub async fn handle_find_user<DB>(db: &DB, name: String) -> Result<User, DatabaseError>
where
    DB: CanReadUserData,
{
    let user = User {
        name,
        password: "".to_string(),
    };
    db.perform(Read(user)).await
}

pub async fn handle_update_user<DB>(db: &DB, user: User) -> Result<User, DatabaseError>
where
    DB: CanUpdateUserData,
{
    db.perform(Update(user)).await
}

pub async fn handle_delete_user<DB>(db: &DB, name: String) -> Result<(), DatabaseError>
where
    DB: CanDeleteUserData,
{
    let u = User {
        name,
        password: "".to_string(),
    };
    db.perform(Delete(u)).await
}

pub async fn display_db_content(con: &SQLite) {

    let users = sqlx::query!(r#"SELECT name, password from users"#)
        .fetch_all(con)
        .await
        .map_err(|e| e);
    
    println!("DBContent: ");
    for u in users {
        println!("{}", u);
    }
    println!();

    let count = sqlx::query!(r#"SELECT COUNT(*) FROM users"#)
        .fetch_one(con)
        .await
        .map_err(|e| e);

    println!("Count: {}\n", count[0].as_integer().unwrap());

}

pub async fn get_db_content(con: &SQLite) -> Result<Vec<User>, DatabaseError> {

    let res = sqlx::query!(r#"SELECT name, password from users"#)
        .fetch_all(con)
        .await
        .map_err(|e| e);

    let mut users = Vec::<User>::new();

    for u in res {
        users.push(u);
    }

    Ok(users)
}
