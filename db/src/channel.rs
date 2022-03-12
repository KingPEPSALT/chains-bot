use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Channels")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub channel_id: i32,
    pub guild_id: i32,
    pub mirror_to_channel_id: Option<i32>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::guild::Entity",
        from = "Column::GuildId",
        to = "super::guild::Column::GuildId"
    )]
    Guild,
}

impl Related<super::guild::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guild.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


