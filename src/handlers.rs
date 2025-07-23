use actix_web::{web, HttpResponse, Responder};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, DatabaseConnection};
use crate::entities::{prelude::Users, users};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

pub async fn create_user(
    user_data: web::Json<NewUser>,
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let new_user = users::ActiveModel {
        username: Set(user_data.username.clone()),
        password: Set(user_data.password.clone()),
        email: Set(user_data.email.clone()),
        ..Default::default()
    };

    match new_user.insert(db.get_ref()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Error al crear usuario: {}", e);
            HttpResponse::InternalServerError().body("Error al crear usuario")
        }
    }
}
