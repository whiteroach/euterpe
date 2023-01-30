pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20230126_164652_create_pictures_table;
mod m20230126_165306_create_genres_table;
mod m20230126_165620_create_playlists_table;
mod m20230130_132523_create_tracks_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20230126_164652_create_pictures_table::Migration),
            Box::new(m20230126_165306_create_genres_table::Migration),
            Box::new(m20230126_165620_create_playlists_table::Migration),
            Box::new(m20230130_132523_create_tracks_table::Migration),
        ]
    }
}
