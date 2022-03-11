use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Members")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub memberId: i32,
    pub guildId: i32,
    pub userId: i32,
    pub watchChannelId: Option<i32>
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::guild::Entity",
        from = "Column::GuildId",
        to = "super::guild::Column::GuildId"
    )]
    Guild,

    #[sea_orm(has_many = "super::warn::Entity")]
    Warn
}
impl Related<super::guild::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guild.def()
    }
}
impl Related<super::warn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Warn.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
