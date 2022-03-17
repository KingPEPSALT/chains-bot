use std::env;

use db::{
    guild::*,
    sea_orm::{Schema, Database, ConnectionTrait}
};

use sea_schema::migration::{
    sea_query::*,
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
        let con = Database::connect(env::var("DATABASE_URL").unwrap()).await?;
        let builder = con.get_database_backend();
        manager.create_table(
            Schema::new(builder).create_table_from_entity(db::guild::Entity)
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Entity).to_owned())
        .await
    }


}
