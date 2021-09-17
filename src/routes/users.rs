use crate::capabilities::{handle_find_all_users, handle_find_user, handle_create_user};
use crate::database::Database;
use actix_web::web::{self};
use actix_web::{Responder, HttpResponse};
use crate::domain::model::{FormData, User};

pub async fn get_user(user: web::Path<String>, pool: web::Data<Database>) -> impl Responder {
    let parsed_user: String = user.into_inner();
    let db = pool.get_ref();
    let u = handle_find_user(db, parsed_user)
        .await
        .expect("Failed to find user");
    serde_json::to_string(&u).unwrap()
}

pub async fn get_all_users(pool: web::Data<Database>) -> impl Responder {
    let db = pool.get_ref();
    let users = handle_find_all_users(db)
        .await
        .expect("Failed to find users");
    serde_json::to_string(&users).unwrap()
}

pub async fn create_new_user(form: web::Form<FormData>, pool: web::Data<Database>) -> impl Responder {

    let db = pool.get_ref();

    let newuser: User = User {name: form.name.clone(), password: form.password.clone()};
    let r = handle_create_user(db, newuser).await;

    match r {
        Ok(_u) => HttpResponse::Ok().finish(),
        _ => HttpResponse::BadRequest().finish()
    }

}