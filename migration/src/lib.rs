pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20230126_164652_create_pictures_table;
mod m20230126_165306_create_genres_table;
mod m20230126_165620_create_playlists_table;
mod m20230130_132523_create_tracks_table;
mod m20230130_153942_create_labels_table;
mod m20230204_232031_create_bands_table;
mod m20230206_151400_create_albums_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20230126_164652_create_pictures_table::Migration),
            Box::new(m20230126_165306_create_genres_table::Migration),
            Box::new(m20230126_165620_create_playlists_table::Migration),
            Box::new(m20230130_132523_create_tracks_table::Migration),
            Box::new(m20230130_153942_create_labels_table::Migration),
            Box::new(m20230204_232031_create_bands_table::Migration),
            Box::new(m20230206_151400_create_albums_table::Migration),
        ]
    }
}
