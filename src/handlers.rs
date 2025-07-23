use actix_web::{web, HttpResponse, Responder};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, DatabaseConnection};
use crate::entities::{prelude::Users, users};
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::Database;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_create_user_exito() {
        let db_url = std::env::var("DATABASE_URL").expect("Falta DATABASE_URL");
        let db = Database::connect(&db_url).await.expect("DB err");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db.clone()))
                .route("/users", web::post().to(create_user)),
        )
        .await;

        #[derive(Deserialize, Serialize)]  
        pub struct NewUser {
            pub username: String,
            pub password: String,
            pub email: String,
        }


        let req = test::TestRequest::post()
            .uri("/users")
            .set_form(&NewUser {
                username: "tester".into(),
                password: "1234".into(),
                email: "tester@example.com".into(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
    }
}
