use db::channel::*;
use db::*;

use sea_schema::migration::{
    sea_query::{self, *},
    *,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_channel_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
            .table(Entity)
            .if_not_exists()
            .col(ColumnDef::new(Column::ChannelId)
                .integer()
                .not_null()
                .primary_key())
            .col(ColumnDef::new(Column::GuildId)
                .not_null()
            )
        .to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Entity).to_owned())
        .await
    }
}