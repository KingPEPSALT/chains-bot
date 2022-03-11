use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Guilds")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub guildId: i32,
    pub isCompliant: i32,
    pub snapChannelId: Option<i32>,
    pub warnChannelId: Option<i32>,
    pub moderationRoleId: Option<i32>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{
    #[sea_orm(has_many = "super::member::Entity")]
    Member,
    #[sea_orm(has_many = "super::member::Channel")]
    Channel,
    #[sea_orm(has_many = "super::member::Warn")]
    Warn
}

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}
impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}
impl Related<super::warn::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Warn.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}