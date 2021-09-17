use crate::capabilities::{handle_create_user, handle_find_all_users, handle_find_user};
use crate::database::Database;
use crate::domain::model::{FormData, User};
use actix_web::web::{self};
use actix_web::{HttpResponse, Responder};

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

pub async fn create_new_user(
    form: web::Form<FormData>,
    pool: web::Data<Database>,
) -> impl Responder {
    let db = pool.get_ref();

    let new_user: User = User {
        name: form.name.clone(),
        password: form.password.clone(),
    };
    let found = handle_find_user(db, new_user.name.clone()).await;
    if found.is_ok() {
        return HttpResponse::Conflict().finish();
    }

    let r = handle_create_user(db, new_user).await;

    match r {
        Ok(u) => {
            let body = serde_json::to_string_pretty(&u).unwrap();
            HttpResponse::Created()
                .content_type("application/json")
                .body(body)
        }
        _ => HttpResponse::NotFound().finish(),
    }
}
