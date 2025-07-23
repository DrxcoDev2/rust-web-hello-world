use sea_orm::{Database, DbConn};
use std::env;
use dotenvy::dotenv;

/// Inicializa y retorna una conexión a la base de datos.
pub async fn init_db() -> DbConn {
    dotenv().ok(); // Cargar variables desde .env

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL no encontrada en .env");

    Database::connect(&database_url)
        .await
        .expect("Error al conectar con la base de datos")
}
