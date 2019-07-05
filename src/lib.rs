#![warn(missing_docs)]
//! # cqs
//!
//! `cqs` is an initialism for Candidate Qualification System

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

/// Establishes a connection with the `DATABASE_URL` environment variable
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
