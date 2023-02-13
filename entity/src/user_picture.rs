//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_picture")]
pub struct Model {
    pub user_id: i32,
    pub picture_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    Pictures,
    Users,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity).from(Column::user_id
            ).to(super::users::Column::UserId).into(),
            Self::Pictures => Entity::belongs_to(super::pictures::Entity).from(Column::picture_id).to(super::pictures::Column::PictureId).into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
