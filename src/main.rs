// Source: https://web.archive.org/web/20180120000131/http://www.zsck.co/writing/capability-based-apis.html
use capabilityapi::capabilities::{
    display_db_content, get_db_content, handle_delete_user, handle_find_user, handle_save_user,
    handle_update_user, SQLite,
};
use capabilityapi::model::User;
use sqlx::SqlitePool;

use actix_web::{web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!\n");

    //let connection = SqlitePool::open(":memory:").unwrap();
    let connection = SqlitePool::connect("sqlite:cap.db")
        .await
        .expect("Failed to get db con");
    sqlx::migrate!("./migrations")
        .run(&connection)
        .await
        .expect("Failed to run migrations");

    let db = SQLite { db: connection };

    let user = User {
        name: "Ollie".to_string(),
        password: "pffpff".to_string(),
    };

    let u = handle_save_user(&db, user)
        .await
        .expect("Failed to save user");

    println!("Saved:");
    println!("{}\n", u);

    let mut boisy = handle_find_user(&db, "boisy".to_string())
        .await
        .expect("Failed to find user");
    println!("Found:");
    println!("{}\n", &boisy);

    boisy.password = "WoofWoof".to_string();

    let updated = handle_update_user(&db, boisy)
        .await
        .expect("Failed to update user");
    println!("Updated:");
    println!("{}\n", updated);

    println!("Deleted:");
    display_db_content(&db).await;
    handle_delete_user(&db, "kenneth".to_string())
        .await
        .expect("Failed to delete user");
    display_db_content(&db).await;

    let pool = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .route("/users/", web::get().to(get_users))
            //.route("/users/{id}", web::get().to(get_user))
            .app_data(pool.clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/*
async fn get_user<DB>(user: web::Path<String>, pool: web::Data<SQLite>) -> impl Responder
where DB: CanReadUserData,
{
    let parseduser: String = user.into_inner();
    let u = handle_find_user(&pool, parseduser).unwrap();
    let json = serde_json::to_string(&u).unwrap();
    format!("{}", json)
}
*/

async fn get_users(pool: web::Data<SQLite>) -> impl Responder {
    let users: Vec<User> = get_db_content(&pool)
        .await
        .expect("failed to get db content");

    let json = serde_json::to_string(&users).unwrap();
    format!("{}", json)
}
