use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Warns")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub warnId: i32,
    pub memberId: i32,
    pub reason: Option<String>,
    pub byUserId: i32,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::member::Entity",
        from = "Column::memberId",
        to = "super::member::Column::memberId"
    )]
    Member
}
impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}