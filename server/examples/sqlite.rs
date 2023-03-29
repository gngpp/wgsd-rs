use sea_orm::{ConnectionTrait, Database, DbBackend, DbConn, DbErr, Schema};
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{ColumnDef, SqliteQueryBuilder, Table, TableCreateStatement};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // Connecting SQLite
    let db = Database::connect("sqlite:/Users/gngpp/CLionProjects/wgsdc/db.sqlite").await?;
    // Setup Schema helper
    let schema = Schema::new(DbBackend::Sqlite);


    Ok(())
}
