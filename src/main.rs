use actix_web::{
    web::{self},
    App, HttpServer, Responder,
};
use actix_web::middleware::Logger;
use capabilityapi::capabilities::{handle_find_user, handle_find_all_users, Database};
use sqlx::Pool;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!\n");
    let connection = Pool::connect("sqlite:cap.db")
        .await
        .expect("Failed to get db");
    
    std::env::set_var("RUST_LOG", "actix_web=info");
    let mut log = env_logger::Builder::from_default_env();
    log.init();

    sqlx::migrate!("./migrations")
        .run(&connection)
        .await
        .expect("Failed to run migrations");

    let db = Database { db: connection };

    let pool = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/users/", web::get().to(get_all_users))
            .route("/users/{id}", web::get().to(get_user))
            .app_data(pool.clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_user(user: web::Path<String>, pool: web::Data<Database>) -> impl Responder {
    let parseduser: String = user.into_inner();
    let db = pool.get_ref();
    let u = handle_find_user(db, parseduser)
        .await
        .expect("Failed to find user");
    serde_json::to_string(&u).unwrap()
}

async fn get_all_users(pool: web::Data<Database>) -> impl Responder {
    let db = pool.get_ref();
    let users = handle_find_all_users(db).await.expect("Faild to find users");
    serde_json::to_string(&users).unwrap()
}
