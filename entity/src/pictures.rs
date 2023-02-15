//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "pictures")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub picture_id: i32,
    pub path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

//MANY-TO-MANY
impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        super::album_picture::Relation::Albums.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::album_picture::Relation::Pictures.def().rev())
    }
}
impl Related<super::bands::Entity> for Entity {
    fn to() -> RelationDef {
        super::band_picture::Relation::Bands.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::band_picture::Relation::Pictures.def().rev())
    }
}
impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_picture::Relation::Users.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_picture::Relation::Pictures.def().rev())
    }
}


impl ActiveModelBehavior for ActiveModel {}
