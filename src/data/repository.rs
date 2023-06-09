use diesel::{prelude::*, result::Error, SqliteConnection as Conn};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

use dotenvy::dotenv;
use std::env;

use super::models::{House, NewHouse};
use super::schema::houses::dsl::*;

pub struct RepositoryError;

impl RepositoryError {
    fn get(_: Error) -> Self {
        RepositoryError
    }
}

fn run_migrations(conn: &mut Conn) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

pub struct Repository {
    conn: Conn,
}

impl Repository {
    pub fn new() -> Self {
        dotenv().ok();
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut conn = Conn::establish(&url).expect(&format!("Error connecting to {}", url));
        run_migrations(&mut conn);
        Repository { conn }
    }

    pub fn find_all(&mut self) -> Result<Vec<House>, RepositoryError> {
        // TODO Pagination should be implemented
        houses
            .order(id.asc())
            .load::<House>(&mut self.conn)
            // TODO Errors should be better reported
            .map_err(RepositoryError::get)
    }

    pub fn create(&mut self, new_house: &NewHouse) -> Result<House, RepositoryError> {
        let result = diesel::insert_into(houses)
            .values(new_house)
            .execute(&mut self.conn);
        if result == Ok(1) {
            return houses
                .order(id.desc())
                .first(&mut self.conn)
                // TODO Errors should be better reported
                .map_err(RepositoryError::get);
        }
        Err(RepositoryError)
    }

    pub fn update(&mut self, house: &House) -> Result<bool, RepositoryError> {
        let result = diesel::update(houses.find(house.id))
            .set(house)
            .execute(&mut self.conn);
        if result == Ok(1) {
            Ok(true)
        } else {
            // TODO Errors should be better reported
            Err(RepositoryError)
        }
    }

    pub fn delete(&mut self, houseid: i32) -> Result<bool, RepositoryError> {
        let result = diesel::delete(houses.find(houseid)).execute(&mut self.conn);
        if result == Ok(1) {
            Ok(true)
        } else {
            // TODO Errors should be better reported
            Err(RepositoryError)
        }
    }
}
