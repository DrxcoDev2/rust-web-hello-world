use dotenv::dotenv;
use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use tera::{Tera, Context};
use entities::users;

mod entities;
mod handlers;

use handlers::{create_user, NewUser};

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("title", "Mi Web en Vivo");
    ctx.insert("message", "¡Hola con recarga automática!");

    match tera.render("index.html.tera", &ctx) {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(err) => {
            eprintln!("Error al renderizar plantilla: {}", err);
            HttpResponse::InternalServerError().body("Error al renderizar plantilla")
        }
    }
}

async fn list_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    match users::Entity::find().all(db.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error al obtener usuarios"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let tera = Tera::new("templates/**/*").expect("Error cargando plantillas");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL no definida");
    let db = Database::connect(&database_url).await.expect("Error conectando DB");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(db.clone()))
            .route("/", web::get().to(index))
            .route("/users", web::get().to(list_users))
            .route("/users", web::post().to(create_user)) // <-- añadir usuario
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
