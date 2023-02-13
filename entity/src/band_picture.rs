//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "band_picture")]
pub struct Model {
    pub band_id: i32,
    pub picture_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // #[sea_orm(
    //     belongs_to = "super::bands::Entity",
    //     from = "Column::BandId",
    //     to = "super::bands::Column::BandId",
    //     on_update = "NoAction",
    //     on_delete = "NoAction"
    // )]
    Bands,
    // #[sea_orm(
    //     belongs_to = "super::pictures::Entity",
    //     from = "Column::PictureId",
    //     to = "super::pictures::Column::PictureId",
    //     on_update = "NoAction",
    //     on_delete = "NoAction"
    // )]
    Pictures,
}


impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Bands => Entity::belongs_to(super::bands::Entity).from(Column::band_id
            ).to(super::albums::Column::BandId).into(),
            Self::Pictures => Entity::belongs_to(super::pictures::Entity).from(Column::picture_id).to(super::labels::Column::PictureId).into(),
        }
    }
}

// impl Related<super::bands::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Bands.def()
//     }
// }

// impl Related<super::pictures::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Pictures.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
