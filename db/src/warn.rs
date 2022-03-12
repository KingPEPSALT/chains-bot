use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Warns")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub warn_id: i32,
    pub member_id: i32,
    pub reason: Option<String>,
    pub by_user_id: i32,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::member::Entity",
        from = "Column::MemberId",
        to = "super::member::Column::MemberId"
    )]
    Member
}
impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}