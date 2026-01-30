use sea_orm::entity::prelude::*;
use sea_orm::Set;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub slug: String,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::post_tag::Entity")]
    PostTags,
}

impl Related<super::post_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostTags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            created_at: Set(chrono::Utc::now().to_rfc3339()),
            ..ActiveModelTrait::default()
        }
    }
}
