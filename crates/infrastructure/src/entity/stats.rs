use sea_orm::entity::prelude::*;

/// Global visitor statistics (singleton table with id=1)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "visit_stats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32, // Always 1 for singleton record
    pub total_visits: i64,
    pub today_visits: i64,
    pub last_updated: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
