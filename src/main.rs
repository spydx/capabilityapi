use actix_web::middleware::Logger;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use capabilityapi::configuration::{DatabaseSettings, Settings};
use capabilityapi::database::Database;
use capabilityapi::routes;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, Capability world!\n");

    std::env::set_var("RUST_LOG", "actix_web=info");
    let mut log = env_logger::Builder::from_default_env();
    log.init();

    let configuration = Settings {
        database: DatabaseSettings {
            name: "sqlite:cap.db".to_string(),
        },
    };

    let db = Database::build(configuration)
        .await
        .expect("Failed to create database");

    let pool = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/users/", web::get().to(routes::users::get_all_users))
            .route("/users/", web::post().to(routes::users::create_new_user))
            .route("/users/{id}", web::get().to(routes::users::get_user))
            .app_data(pool.clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
