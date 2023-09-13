use diesel::{PgConnection, r2d2::{self, ConnectionManager}};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn init_db_pool(url: &str) -> Pool {
    info!("Migrating and configuring database...");

    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}

pub fn  run_migration(conn: &mut Connection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}