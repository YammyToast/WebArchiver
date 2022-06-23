#[macro_use]
extern crate diesel;
extern crate dotenv;


use core::ops::*;

// Database Imports
use diesel::{pg::PgConnection, Connection, QueryDsl};
use std::env;
use self::diesel::prelude::*;

// Schema Imports
mod schema;
mod model;
mod db;
use crate::model::*;

// Reqwest Imports
use error_chain::{error_chain, ChainedError};
use std::io::Read;
use reqwest::{Client, Response};

// http Imports
mod http;

// Error-chain Macro
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


fn main() {
    //use crate::schema::sitepages::dsl::*;
    use crate::schema::siteindexs::dsl::*;


    // Matches a connection case and panics if no connection is established.
    let connection = match establish_connection() {
        None => panic!("No Connection Established"),
        Some(val) => val  
    };

    
    // Example insert function.
    match db::siteindex::db_add_site_index(&connection, "yammy", ".me") {
        None => panic!("Insert Statement Failed"),
        Some(val) => println!("Successfully inserted into Siteindex: {:?}, {:?}", val.name.unwrap(), val.domain.unwrap())
        
    };
    
    // let sitepages_results = sitepages.limit(3)
    //                             .load::<Sitepage>(&connection)
    //                             .expect("Error Loading Posts");
    // for item in sitepages_results {
        //     println!("{} | {} | {:?}", item.pageid, item.siteid, item.texturl);
        
        // }
        
    
    let siteindex_results = siteindexs
    .load::<Siteindex>(&connection)
    .expect("Error Loading Posts");
    
    // Example select function.
    for item in siteindex_results {
        println!("{:?} | {:?} | {:?}", item.siteid, item.name.unwrap(), item.domain.unwrap());

    }

    let request_url = "https://github.com/YammyToast/WebArchiver".to_string();
    // URLParse Error
    // let request_url = "this.is.not.a..valid.ur//l".to_string();
    // Response Error
    //let request_url = "https://www.wjgoajgowhoahgoehgoajgoejao.com/".to_string();

    // GET from URL
    let response = match http::try_request_from_url(&request_url) {
        Ok(val) => val,
        Err(http::RequestError { fault, msg }) => panic!("Error of type: \"{:?}\" occured, msg: {}", fault, msg)
    };

    


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

