use actix_web::web::{self};
use actix_web::{Responder};
use crate::database::Database;
use crate::capabilities::{handle_find_user, handle_find_all_users};

pub async fn get_user(user: web::Path<String>, pool: web::Data<Database>) -> impl Responder {
    let parseduser: String = user.into_inner();
    let db = pool.get_ref();
    let u = handle_find_user(db, parseduser)
        .await
        .expect("Failed to find user");
    serde_json::to_string(&u).unwrap()
}

pub async fn get_all_users(pool: web::Data<Database>) -> impl Responder {
    let db = pool.get_ref();
    let users = handle_find_all_users(db)
        .await
        .expect("Faild to find users");
    serde_json::to_string(&users).unwrap()
}