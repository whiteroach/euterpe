//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "labels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub label_id: i32,
    pub name: String,
    pub info: Option<Json>,
    pub owned: bool,
    pub year: Option<Date>,
    pub genre_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::albums::Entity")]
    Albums,
    #[sea_orm(has_many = "super::band_label::Entity")]
    BandLabel,
    #[sea_orm(
        belongs_to = "super::genres::Entity",
        from = "Column::GenreId",
        to = "super::genres::Column::GenreId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Genres,
    #[sea_orm(has_many = "super::user_label::Entity")]
    UserLabel,
}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Albums.def()
    }
}

// impl Related<super::band_label::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::BandLabel.def()
//     }
// }
//MANY-TO-MANY
impl Related<super::bands::Entity> for Entity {
    fn to() -> RelationDef {
        super::band_label::Relation::Bands.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::band_label::Relation::Labels.def().rev())
    }
}
//MANY-TO-MANY
impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_label::Relation::Users.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_label::Relation::Labels.def().rev())
    }
}

impl Related<super::genres::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genres.def()
    }
}

// impl Related<super::user_label::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::UserLabel.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
