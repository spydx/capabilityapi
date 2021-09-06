// Source: https://web.archive.org/web/20180120000131/http://www.zsck.co/writing/capability-based-apis.html
use capabilityapi::capabilities::{SQLite, display_db_content, get_db_content, handle_delete_user, handle_find_user, handle_save_user, handle_update_user};
use capabilityapi::model::User;

use actix_web::{web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!\n");

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

    println!("Saved:");
    println!("{}\n", u);

    let mut boisy = handle_find_user(&db, "boisy".to_string()).unwrap();
    println!("Found:");
    println!("{}\n", &boisy);

    boisy.password = "WoofWoof".to_string();

    let updated = handle_update_user(&db, boisy).unwrap();
    println!("Updated:");
    println!("{}\n", updated);

    display_db_content(&db);
    handle_delete_user(&db, "kenneth".to_string()).unwrap();
    display_db_content(&db);

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
    let users: Vec<User> = get_db_content(&pool).unwrap();

    let json = serde_json::to_string(&users).unwrap();
    format!("{}", json)
}
