use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Warns")]
pub struct Model{
    #[sea_orm(primary_key, auto_increment=true)]
    pub warn_id: u32,
    pub guild_id: i64,
    pub user_id: i64,
    pub reason: Option<String>,
    pub by_user_id: String,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::member::Entity",
        from = "(Column::GuildId, Column::UserId)",
        to = "(super::member::Column::GuildId, super::member::Column::UserId)"
    )]
    Member
}
impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}