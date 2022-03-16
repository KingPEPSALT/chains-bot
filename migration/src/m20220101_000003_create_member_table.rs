use db::{
    member::*,
    *
};
use sea_schema::migration::{
    sea_query::*,
    *,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000003_create_member_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
            .table(member::Entity)
            .if_not_exists()
            .col(ColumnDef::new(member::Column::MemberId)
                .integer()
                .not_null()
                .primary_key()
                .auto_increment())
            .col(ColumnDef::new(member::Column::GuildId)
                .integer()
                .not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_Guild_Id")
                        .from(Entity, Column::GuildId)
                        .to(guild::Entity, guild::Column::GuildId)
                )
            .col(ColumnDef::new(Column::UserId)
                .integer()
                .not_null())
            .col(ColumnDef::new(Column::WatchChannelId)
                .integer())
            .to_owned()
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(member::Entity).to_owned())
        .await
    }
}