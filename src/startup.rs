use crate::auth;
use crate::configuration::Settings;
use crate::database::Database;
use crate::routes;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let mut log = env_logger::Builder::from_default_env();

        let logging = match std::env::var("RUST_LOG") {
            Err(_) => {
                std::env::set_var("RUST_LOG", "actix_web=info");
                std::env::var("RUST_LOG").unwrap()
            }
            Ok(_) => std::env::var("RUST_LOG").unwrap(),
        };

        if !logging.eq("actix_web=info") {
            log.init();
        }

        let db = Database::build(&configuration)
            .await
            .expect("Failed to create database");

        let address = &configuration.server_string();

        let listener = TcpListener::bind(address).expect("Failed to allocate host address");
        let port = listener.local_addr().unwrap().port();

        let pool = web::Data::new(db);
        let server = configure(listener, pool)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn configure(address: TcpListener, database: Data<Database>) -> Result<Server, std::io::Error> {
    let middleware = HttpAuthentication::bearer(auth::token_validator);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware.clone())
            .wrap(Logger::default())
            .route("/users/", web::get().to(routes::users::get_all_users))
            .route("/users/", web::post().to(routes::users::create_new_user))
            .route("/users/{id}", web::get().to(routes::users::get_user))
            .app_data(database.clone())
    })
    .listen(address)?
    .run();

    Ok(server)
}
