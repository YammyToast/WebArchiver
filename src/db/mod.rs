// Diesel Imports
use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods};


pub mod siteindex;
pub mod sitepages;
pub mod pagerecord;


pub fn create_site_index<'a>(connection: &PgConnection, name: &'a str, domain: &'a str) -> Result<(), String> {
    // Check for existing record for this website name.
    match siteindex::db_get_records(connection, name) {
        // Check function can't connect to the DB.
        Err(_) => Err("Error connecting to DB for existing site check".to_string()),
        // The check function was able to return a Vec of records.
        Ok(list) => match list.len() {
            // If there is one or more records, i.e. the site already exists,
            // then don't add a duplicate.
            1.. => Err(format!("Site Index already exists with ID: {}", list.first().unwrap().siteid).to_string()),
            // If no records exist for this name, allow name to be added to DB.
            0 => match siteindex::db_add_site_index(connection, name, domain) {
                None => Err("Site Index couldn't be added to DB succesfully".to_string()), 
                Some(_) => Ok(())
            },
            // Only thrown if a list is returned, but there is neither 0, or 1.. items.
            _ => Err("Malformed list object from existing index check".to_string())
        }
    }

}

pub fn create_site_page<'a>(connection: &PgConnection, indexid: i32, texturl: &'a str) -> Result<(), String> {
    match siteindex::db_check_id_exists(connection, indexid) {
        Err(e) => Err("Error connecting to DB for existing index check".to_string()),
        Ok(exists) => match exists {
            false => Err(format!("No Site exists in DB with given ID: {:?}", indexid).to_string()),
            // Checking for both existing records for the same id AND texturl can't be done easily in Diesel,
            // so it is done serially => get all records for given id, then iterate over that list for given url.
            true => match sitepages::db_get_records(connection, indexid) {
                Err(_) => Err("Error getting records from DB for site index check".to_string()),
                Ok(list) => match list.iter().any(|record| record.texturl.as_ref().unwrap() == texturl) {
                    true => Err("Site Page for this URL already exists in DB".to_string()),
                    false => {sitepages::db_add_site_page(connection, indexid, texturl); Ok(())}

                } 
            }

        }
    }
}

// TODO: IMPROVE ERROR OPTIONS.
pub fn create_page_record(connection: &PgConnection, pageid: i32) -> Result<(), String> {
    // CHANGE THIS TO BE A PAGE REFERENCE
    match siteindex::db_check_id_exists(connection, pageid) {
        // If somehow there is an error connecting to the DB at this point in runtime
        Err(_) => Err("Error Connecting to DB".to_string()),
        // If there is no error connecting does the id exist in db.
        Ok(exists) => match exists{
            // If it don't exist then give up
            false => Err(format!("No Page exists in DB with given ID: {:?}", pageid).to_string()),
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