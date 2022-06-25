#[macro_use]
extern crate diesel;
extern crate dotenv;


// Database Imports
use diesel::{pg::PgConnection, Connection};
use std::env;
use self::diesel::prelude::*;

// Schema Imports
mod schema;
mod model;
mod db;
use crate::model::*;

// http Imports
mod http;

// Reqwest Imports
use error_chain::{error_chain};


// Error-chain Macro
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


fn main() {
    // Matches a connection case and panics if no connection is established.
    let connection = match establish_connection() {
        None => panic!("No Connection Established"),
        Some(val) => val  
    };

    let request_url = "https://datatracker.ietf.org/doc/html/rfc4122#page-14".to_string();    
    //let request_url = "https://github.com/YammyToast/WebArchiver".to_string();
    // URLParse Error
    // let request_url = "this.is.not.a..valid.ur//l".to_string();
    // Response Error
    //let request_url = "https://www.wjgoajgowhoahgoehgoajgoejao.com/".to_string();

    // GET from URL
    // let response = match http::try_request_from_url(&request_url) {
    //     Ok(val) => val,
    //     Err(http::RequestError { fault, msg }) => panic!("Error of type: \"{:?}\" occured, msg: {}", fault, msg)
    // };


    match db::create_site_index(&connection, "yammy", ".me") {
        Err(e) => println!("{:?}", e),
        Ok(_) => {}

    }
    

    match db::siteindex::db_get_records(&connection, "yammy") {
        Err(_) => println!("got here for some reason"),
        Ok(list) => {
            for record in list {
                println!("{:?} | {:?} | {:?}", record.siteid, record.name.unwrap(), record.domain.unwrap());
            }
        }
    }


    // match db::create_page_record(&connection, 1) {
    //     Err(e) => println!("{:?}", e),
    //     Ok(_) => {}
    // }

    match db::create_site_page(&connection, 1, "swag.com/antiswag") {
        Err(e) => println!("{:?}", e),
        Ok(_) => {}

    }

    match db::sitepages::db_get_records(&connection, 1) {
        Err(_) => println!(),
        Ok(list) => {
            println!("\n\n");
            for record in list {
                println!("{:?} | {:?} | {:?}", record.pageid, record.siteid, record.texturl.unwrap())
            }
        }
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

