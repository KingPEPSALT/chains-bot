use db::{
    channel::*,
    *
};

use sea_schema::migration::{
    sea_query::*,
    *,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_create_channel_table"
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
                .integer()
                .not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_Guild_Id")
                        .from(Entity, Column::GuildId)
                        .to(guild::Entity, guild::Column::GuildId)
                )
            .col(ColumnDef::new(Column::MirrorToChannelId)
                .integer())
        .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Entity).to_owned())
        .await
    }
}