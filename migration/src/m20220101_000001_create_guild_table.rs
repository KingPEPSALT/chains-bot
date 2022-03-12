
use db::guild::*;
use db::*;

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


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
        .table(Entity)
        .if_not_exists()
        .col(ColumnDef::new(Column::GuildId)
            .integer()
            .not_null()
            .auto_increment()
            .primary_key())
        .col(ColumnDef::new(Column::IsCompliant).boolean().not_null())
        .col(ColumnDef::new(Column::ModerationRoleId).integer())
        .col(ColumnDef::new(Column::SnapChannelId).integer())
        .col(ColumnDef::new(Column::WarnChannelId).integer())
        .foreign_key(
            ForeignKey::create()
            .name("FK_Snap_Channel_Id")
            .from(Entity, Column::SnapChannelId)
            .to(channel::Entity, channel::Column::ChannelId)
        )
        .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Entity).to_owned())
        .await
    }


}
