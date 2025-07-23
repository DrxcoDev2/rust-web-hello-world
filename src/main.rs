mod db;

use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use tera::{Tera, Context};
use sea_orm::DbConn;

async fn index() -> impl Responder {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error al cargar plantillas: {}", e);
            return HttpResponse::InternalServerError().body("Error cargando plantillas");
        }
    };

    let mut ctx = Context::new();
    ctx.insert("title", "Mi Web en Vivo");
    ctx.insert("message", "¡Hola con recarga automática!");

    match tera.render("index.html.tera", &ctx) {
        Ok(rendered) => HttpResponse::Ok().content_type("text/html").body(rendered),
        Err(err) => {
            eprintln!("Error al renderizar plantilla: {}", err);
            HttpResponse::InternalServerError().body("Error al renderizar plantilla")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DbConn = db::init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Inyecta la conexión en las rutas
            .route("/", web::get().to(index))
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
