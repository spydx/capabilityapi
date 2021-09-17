use actix_web::middleware::Logger;
use actix_web::{
    web::{self},
    App, HttpServer,
};
use capabilityapi::configuration::get_configuration;
use capabilityapi::database::Database;
use capabilityapi::routes;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, Capability world!\n");

    std::env::set_var("RUST_LOG", "actix_web=info");
    let mut log = env_logger::Builder::from_default_env();
    log.init();

    let configuration = get_configuration().expect("Failed to get config");

    let db = Database::build(&configuration)
        .await
        .expect("Failed to create database");

    let address = &configuration.server_string();
    let pool = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/users/", web::get().to(routes::users::get_all_users))
            .route("/users/", web::post().to(routes::users::create_new_user))
            .route("/users/{id}", web::get().to(routes::users::get_user))
            .app_data(pool.clone())
    })
    .bind(address)?
    .run()
    .await
}
