use db::{    
    warn::*,
    *
};

use sea_schema::migration::{
    sea_query::*,
    *,
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000004_create_warn_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create()
            .table(warn::Entity)
            .col(ColumnDef::new(warn::Column::WarnId)
                .integer()
                .not_null()
                .primary_key()
                .auto_increment())
            .col(ColumnDef::new(warn::Column::MemberId)
                .integer()
                .not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("FK_Member_Id")
                        .from(Entity, Column::MemberId)
                        .to(member::Entity, member::Column::MemberId)
                )
            .col(ColumnDef::new(warn::Column::Reason)
                .text())
            .col(ColumnDef::new(warn::Column::ByUserId)
                .integer()
                .not_null())
            .to_owned()
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Entity).to_owned())
        .await
    }
}