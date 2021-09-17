use crate::database::{Database, DatabaseError};
use crate::domain::model::User;
use async_trait::async_trait;

pub struct Create<T>(pub T);
pub struct Read<T>(pub T);
pub struct ReadAll<T>(pub T);
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

capability!(CanReadUserData for Database,
    composing {Read<String>, User, DatabaseError});

capability!(CanReadAllUserData for Database,
    composing{ ReadAll<()>, Vec<User>, DatabaseError});

capability!(CanCreateUserData for Database,
    composing{ Create<User>, User, DatabaseError});

capability!(CanDeleteUserData for Database,
    composing{ Delete<User>, (), DatabaseError});

#[async_trait]
impl Capability<Read<String>> for Database {
    type Data = User;
    type Error = DatabaseError;

    async fn perform(&self, find_user: Read<String>) -> Result<Self::Data, Self::Error> {
        let userid = find_user.0;
        let record = sqlx::query!(r#"SELECT * FROM users WHERE name = $1"#, userid,)
            .fetch_one(&self.db)
            .await
            .map_err(|e| e);
        let user = match record {
            Ok(r) => User {
                name: r.name.unwrap(),
                password: r.password.unwrap(),
            },
            _ => return Err(DatabaseError),
        };

        Ok(user)
    }
}

pub async fn handle_find_user<DB>(db: &DB, name: String) -> Result<User, DatabaseError>
where
    DB: CanReadUserData,
{
    db.perform(Read(name)).await
}

#[async_trait]
impl Capability<ReadAll<()>> for Database {
    type Data = Vec<User>;
    type Error = DatabaseError;

    async fn perform(&self, _: ReadAll<()>) -> Result<Self::Data, Self::Error> {
        let records = sqlx::query!(r#"SELECT name, password FROM users"#)
            .fetch_all(&self.db)
            .await
            .map_err(|e| e);

        let mut users = vec![];
        for row in records.unwrap().into_iter() {
            let name = row.name.unwrap();
            let password = row.password.unwrap();

            let u = User { name, password };

            users.push(u);
        }

        Ok(users)
    }
}

pub async fn handle_find_all_users<DB>(db: &DB) -> Result<Vec<User>, DatabaseError>
where
    DB: CanReadAllUserData,
{
    db.perform(ReadAll(())).await
}

#[async_trait]
impl Capability<Create<User>> for Database {
    type Data = User;
    type Error = DatabaseError;

    async fn perform(&self, create_user: Create<User>) -> Result<Self::Data, Self::Error> {
        let user = create_user.0;

        let r = sqlx::query!(
            r#"INSERT INTO users (name, password) VALUES ($1, $2)"#,
            user.name,
            user.password
        )
        .execute(&self.db)
        .await
        .map_err(|e| e);
        match r {
            Ok(_) => Ok(user),
            _ => Err(DatabaseError),
        }
    }
}

pub async fn handle_create_user<DB>(db: &DB, createuser: User) -> Result<User, DatabaseError>
where
    DB: CanCreateUserData,
{
    db.perform(Create(createuser)).await
}

#[async_trait]
impl Capability<Delete<User>> for Database {
    type Data = ();
    type Error = DatabaseError;

    async fn perform(&self, delete_user: Delete<User>) -> Result<Self::Data, Self::Error> {
        let user = delete_user.0;

        sqlx::query!(r#"DELETE FROM users WHERE name = $1"#, user.name)
            .execute(&self.db)
            .await
            .map_err(|e| e)
            .unwrap();

        Ok(())
    }
}

pub async fn handle_delete_user<DB>(db: &DB, user_to_delete: User) -> Result<(), DatabaseError>
where
    DB: CanDeleteUserData,
{
    db.perform(Delete(user_to_delete)).await
}

/*

capability!(CanUpdateUserData for SQLite,
    composing { Update<User>, User, DatabaseError});

capability!(CanDeleteUserData for SQLite,
    composing   { Delete<User>, (), DatabaseError});

capability!(CanReadAndChangeData for SQLite,
    composing   { Read<User>, User, DatabaseError},
                { Update<User>, User, DatabaseError});

*/

/*
#[async_trait]
impl Capability<Update<User>> for SQLite {
    type Data = User;
    type Error = DatabaseError;

    async fn perform(&self, updated_user: Update<User>) -> Result<Self::Data, Self::Error> {

        let r = sqlx::query!(
            r#"UPDATE users SET password = $1 WHERE name = $2"#,
            updated_user.0.password,
            updated_user.0.name
        )
        .execute(&mut self.db)
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
        sqlx::query!(
            r#"DELETE FROM users WHERE name = $1"#,
            user_to_delete.0.name
        )
        .execute(&mut self.db)
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
*/

/*
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


pub async fn get_db_content(con: &SQLite) -> Result<Vec<User>, DatabaseError> {
    let mut db = &con;
    let res = sqlx::query(r#"SELECT name, password FROM users"#)
        .fetch_all(&con.db)
        .await
        .map_err(|e| e);

    let users = vec![];
    /*
    for row in res {
        let user = User { name: row.name, password: row.password};
        users.push(user);
    }
    */
    Ok(users)
}
*/
