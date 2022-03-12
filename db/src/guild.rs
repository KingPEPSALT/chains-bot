use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Guilds")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub guild_id: i32,
    pub is_compliant: bool,
    pub snap_channel_id: Option<i32>,
    pub warn_channel_id: Option<i32>,
    pub moderation_role_id: Option<i32>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation{
    #[sea_orm(has_many = "super::member::Entity")]
    Member,
    #[sea_orm(has_many = "super::channel::Entity")]
    Channel,

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


impl ActiveModelBehavior for ActiveModel {}