use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL no encontrada");
    Database::connect(&database_url)
        .await
        .expect("Error al conectar con la base de datos")
}
