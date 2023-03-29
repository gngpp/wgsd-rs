use sea_orm::{ConnectionTrait, DbBackend, DbConn, DbErr, Schema, sea_query};
use sea_orm::sea_query::{ColumnDef, SqliteQueryBuilder, Table, TableCreateStatement};
use crate::model::node;

pub async fn setup_schema(db: &DbConn) -> Result<(), DbErr>{

    use sea_query::*;

    let stmt = Table::create()
        .table(node::Entity)
        .col(
            ColumnDef::new(node::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(node::Column::Relay)
            .integer()
            .not_null()
            .default(Value::Bool(Some(false)))
        )
        .col(ColumnDef::new(node::Column::Name)
            .string()
            .not_null()
            .unique_key()
        )
        .col(ColumnDef::new(node::Column::PublicKey).string())
        .col(ColumnDef::new(node::Column::PrivateKey).string())
        .col(ColumnDef::new(node::Column::ListenPort).string())
        .col(ColumnDef::new(node::Column::AllowedIps).string())
        .col(ColumnDef::new(node::Column::EndpointAllowedIps).string())
        .col(ColumnDef::new(node::Column::PersistentKeepalive).string())
        .col(ColumnDef::new(node::Column::Endpoint).string())
        .col(ColumnDef::new(node::Column::PersistentKeepalive).string())
        .col(ColumnDef::new(node::Column::Mtu).string())
        .col(ColumnDef::new(node::Column::PostUp).string())
        .col(ColumnDef::new(node::Column::PostDown).string())
        .col(ColumnDef::new(node::Column::PreUp).string())
        .col(ColumnDef::new(node::Column::PreDown).string())
        .to_owned();


    let builder = db.get_database_backend();
    let result = db.execute(builder.build(&stmt)).await?;
    println!("Create table node: {:?}", result);

    Ok(())
}