use std::path::PathBuf;

use sea_orm::{sea_query, ConnectionTrait, DbConn, DbErr};

pub mod model;

// initialize database, the database path does not have to exist
pub async fn initialize_database(database_path: PathBuf) -> anyhow::Result<(), DbErr> {
    if database_path.is_file() {
        log::debug!(
            "the {} database file already exists",
            database_path.display()
        );
    } else if database_path.is_dir() {
        panic!("the {} is a directory, not a file", database_path.display())
    } else {
        match tokio::fs::File::create(&database_path).await {
            Ok(_) => {
                log::debug!(
                    "the {} database file has been created",
                    database_path.display()
                );
                log::debug!("begins initializing the database table");
                let db = sea_orm::Database::connect(format!("sqlite:{}", database_path.display()))
                    .await?;
                initialize_table(&db).await?
            }
            Err(e) => {
                panic!("failed to create database file, error: {}", e)
            }
        };
    }
    Ok(())
}

async fn initialize_table(db: &DbConn) -> anyhow::Result<(), DbErr> {
    use model::*;
    use sea_query::*;

    let stmt = Table::create()
        .table(node_relay::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(node_relay::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(node_relay::Column::ParentId).integer())
        .col(
            ColumnDef::new(node_relay::Column::Relay)
                .integer()
                .not_null()
                .default(Value::Bool(Some(false))),
        )
        .col(
            ColumnDef::new(node_relay::Column::Name)
                .string()
                .not_null()
                .unique_key(),
        )
        .col(ColumnDef::new(node_relay::Column::PublicKey).string())
        .col(ColumnDef::new(node_relay::Column::PrivateKey).string())
        .col(ColumnDef::new(node_relay::Column::ListenPort).string())
        .col(ColumnDef::new(node_relay::Column::Dns).string())
        .col(ColumnDef::new(node_relay::Column::AllowedIps).string())
        .col(ColumnDef::new(node_relay::Column::EndpointAllowedIps).string())
        .col(ColumnDef::new(node_relay::Column::Endpoint).string())
        .col(ColumnDef::new(node_relay::Column::PersistentKeepalive).string())
        .col(ColumnDef::new(node_relay::Column::Mtu).string())
        .col(ColumnDef::new(node_relay::Column::PostUp).string())
        .col(ColumnDef::new(node_relay::Column::PostDown).string())
        .col(ColumnDef::new(node_relay::Column::PreUp).string())
        .col(ColumnDef::new(node_relay::Column::PreDown).string())
        .to_owned();

    let builder = db.get_database_backend();
    let result = db.execute(builder.build(&stmt)).await?;
    println!("Create table node: {:?}", result);

    Ok(())
}
