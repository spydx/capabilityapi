use crate::model::User;
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct SQLite {
    pub db: SqlitePool,
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

        let r = sqlx::query!(
            r#"INSERT INTO users (name, password) VALUES ($1, $2)"#,
            save_user.0.name,
            save_user.0.password
        )
        .execute(&mut access)
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
        let userid = find_user.0.name;
        /*let record = sqlx::query_as!(User,
                        r#"SELECT * FROM users WHERE name = $1"#,
                        userid,
                    )
                    .fetch_one(&self.db)
                    .await
                    .map_err(|e| e);

                Ok(record.unwrap())
        //        let user = User { name: record.name , password: record.password };
          //      Ok(user)
                */
        let u = User {
            name: userid,
            password: "somestupidthing".to_string(),
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

        let r = sqlx::query!(
            r#"UPDATE users SET password = $1 WHERE name = $2"#,
            updated_user.0.password,
            updated_user.0.name
        )
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

        sqlx::query!(
            r#"DELETE FROM users WHERE name = $1"#,
            user_to_delete.0.name
        )
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
    let _users = sqlx::query(r#"SELECT * FROM users"#)
        .fetch_all(&con.db)
        .await
        .map_err(|e| e);

    println!("DBContent: ");
    /*
    for row in users {
        let user = User { name: row.name, password: row.password};
        println!("{}", user);
    }*/
    println!();
    /*
    let row = sqlx::query!(r#"SELECT COUNT(*) FROM users"#)
        .fetch_one(&con.db)
        .await
        .map_err(|e| e);
    let count = row.map(|r| r.count).unwrap();
    println!("Count: {}\n", count.unwrap());
    */
}

pub async fn get_db_content(con: &SQLite) -> Result<Vec<User>, DatabaseError> {
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
