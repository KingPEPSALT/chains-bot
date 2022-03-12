use std::borrow::{BorrowMut, Borrow};

use db::guild::*;

use sea_schema::migration::{
    sea_query::{self, *},
    *,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_guild_table"
    }
}
#[derive(Iden)]
pub enum Guild {
    Table,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
        .table(Guild::Table)
        .if_not_exists()
        .col(ColumnDef::new(Column::GuildId)
            .integer()
            .not_null()
            .auto_increment()
            .primary_key())
        .to_owned()
        .col(ColumnDef::new(Column::IsCompliant).boolean().not_null())
        .col(ColumnDef::new(Column::ModerationRoleId).integer())
        .col(ColumnDef::new(Column::SnapChannelId).integer())
        .foreign_key(
            ForeignKey::create()
            .name("FK_Snap_Channel_Id")
            .from(Guild::Table, Column::SnapChannelId)
            .to(db::channel::Entity, db::channel::Column::ChannelId)
        )
        .to_owned()).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Guild::Table).to_owned())
        .await
    }


}
