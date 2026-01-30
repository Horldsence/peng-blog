use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "post_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub post_id: String,
    #[sea_orm(primary_key)]
    pub tag_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Tag,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
