#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{pg::PgConnection, Connection, ConnectionError};
use std::env;


use crate::schema::sitepages::dsl::*;
use crate::model::Sitepage;


mod schema;
mod model;

fn main() {



    // Matches a connection case and panics if no connection is established.
    let connection = match establish_connection() {
        None => panic!("No Connection Established"),
        Some(val) => val  
    };

    let results = sitepages.filter() 
    //sitepages.limit(2).load::<pageid>(&connection).expect("Err");
    


    
    
}


fn establish_connection() -> Option<PgConnection> {
    // Establish dotenv file stream. 
    // Standard ::dotenv call loaded sys env's so this is a verbose workaround.
    dotenv::from_filename(".env").expect("Error locating env file");

    // Parse env stream for DATABASE_URL var.
    let data_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL var not found or invalid.");

    // Create connection struct using database url.
    let connection = PgConnection::establish(&data_url);

    // If there is a connection, return the connection struct, otherwise return none.
    match connection {
        Ok(val) => Some(val),
        _ => None
    }
}
