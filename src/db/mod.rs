// Diesel Imports
use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods};

mod vault;
pub mod siteindex;
pub mod pagerecord;


pub fn create_site_index<'a>(connection: &PgConnection, name: &'a str, domain: &'a str) -> Result<(), String> {
    match siteindex::db_check_existing_name(connection, name) {
        Err(e) => match e {
            Some(index) => Err(format!("Site Index already exists with ID: {}", index).to_string()),
            None => Err("Error connecting to DB for existing name check".to_string())
        },
        Ok(_) => match siteindex::db_add_site_index(connection, name, domain) {
            None => Err("Site Index couldn't be added to DB succesfully".to_string()),
            Some(_) => Ok(())
        }
    }

}

pub fn create_page_index(connection: &PgConnection) {

}

// TODO: IMPROVE ERROR OPTIONS.
pub fn create_page_record(connection: &PgConnection, pageid: i32) -> Result<(), String> {
    match siteindex::db_check_id_exists(connection, pageid) {
        // If somehow there is an error connecting to the DB at this point in runtime
        Err(_) => Err("Error Connecting to DB".to_string()),
        // If there is no error connecting does the id exist in db.
        Ok(exists) => match exists{
            // If it don't exist then give up
            false => Err("No Page exists in DB with given ID".to_string()),
            // If it do exist then go
            true => match pagerecord::db_add_page_record(connection, pageid) {
                    // Page record added successfully

                    // ADD 'VAULT' FUNCTION ON THE SOME ARM
                    Some(val) => Ok(()),
                    
                    // Page record wasn't added successfully
                    None => Err("Page record couldn't be added to DB succesfully.".to_string())
                }
        }

    }

}