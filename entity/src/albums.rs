//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "albums")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub album_id: i32,
    pub title: String,
    pub r#type: Option<String>,
    pub duration: Option<Time>,
    pub info: Option<Json>,
    pub year: Option<Date>,
    pub deleted_at: Option<DateTime>,
    pub label_id: Option<i32>,
    pub genre_id: Option<i32>,
    pub user_id: Option<i32>,
    pub band_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::bands::Entity",
        from = "Column::BandId",
        to = "super::bands::Column::BandId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Bands,
    #[sea_orm(
        belongs_to = "super::genres::Entity",
        from = "Column::GenreId",
        to = "super::genres::Column::GenreId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Genres,
    #[sea_orm(
        belongs_to = "super::labels::Entity",
        from = "Column::LabelId",
        to = "super::labels::Column::LabelId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Labels,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

//MANY-TO-MANY
impl Related<super::pictures::Entity> for Entity {
    fn to() -> RelationDef {
        super::album_picture::Relation::Pictures.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::album_picture::Relation::Albums.def().rev())
    }
}
//MANY-TO-MANY
impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        super::album_track::Relation::Tracks.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::album_track::Relation::Albums.def().rev())
    }
}

impl Related<super::bands::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bands.def()
    }
}

impl Related<super::genres::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genres.def()
    }
}

impl Related<super::labels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Labels.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
