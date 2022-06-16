#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{pg::PgConnection, Connection, QueryDsl};
use std::env;
use self::diesel::prelude::*;

mod schema;
mod model;
mod operations;
use crate::model::*;


fn main() {
    // 
    //use crate::schema::sitepages::dsl::*;
    use crate::schema::siteindexs::dsl::*;


    // Matches a connection case and panics if no connection is established.
    let connection = match establish_connection() {
        None => panic!("No Connection Established"),
        Some(val) => val  
    };

    
    // Example insert function.
    match operations::siteindex::db_Add_Site_Index(&connection, "yammy", ".me") {
        None => panic!("Insert Statement Failed"),
        Some(val) => println!("Successfully inserted into Siteindex: {:?}, {:?}", val.name.unwrap(), val.domain.unwrap())
        
    };
    
    // let sitepages_results = sitepages.limit(3)
    //                             .load::<Sitepage>(&connection)
    //                             .expect("Error Loading Posts");
    
    let siteindex_results = siteindexs
    .load::<Siteindex>(&connection)
    .expect("Error Loading Posts");
    
    // for item in sitepages_results {
        //     println!("{} | {} | {:?}", item.pageid, item.siteid, item.texturl);
        
        // }
        
    // Example select function.
    for item in siteindex_results {
        println!("{:?} | {:?} | {:?}", item.siteid, item.name.unwrap(), item.domain.unwrap());

    }

}


fn establish_connection() -> Option<PgConnection> {
    // Establish dotenv file stream. 
    // Standard ::dotenv call loaded system env's so this is a workaround.
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
