//! This crate is stupid, don't evaluate it's functionality. It is simply a
//! demonstration of an issue.

#[macro_use]
extern crate diesel;

use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use diesel::sql_types::Integer;
use diesel::prelude::*;

pub const DATABASE_URL: &'static str = "demoissue.sqlite3";

fn main() {

    let conn = SqliteConnection::establish(DATABASE_URL)
        .expect(&*format!("Error connectiong to database: {}", DATABASE_URL));

    if let Some(ages) = demo_query(&conn, 1) {
        println!("Ages: {} > {}", ages.age, ages.minimum_age);
    }
}

#[derive(QueryableByName)]
pub struct Ages {
    #[sql_type = "Integer"]
    pub age: i32,
    #[sql_type = "Integer"]
    pub minimum_age: i32,
    #[sql_type = "String"]
    pub somestuff: String,
}

pub fn demo_query(conn: &SqliteConnection, id: i32) -> Option<Ages> {
    diesel::sql_query(include_str!("demo_query.sql"))
        .bind::<Integer, _>(id)
        .get_result::<Ages>(conn)
        .ok()
}
