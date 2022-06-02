#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{pg::PgConnection, Connection};
use std::env;


mod schema;
//mod model;

fn main() {
    dotenv::from_filename(".env").expect("Error locating env file");
    let data_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL var not found or invalid.");
    let connection = PgConnection::establish(&data_url)
        .expect("Database connection could not be established.");
    
    
    
    
}

