// Diesel Imports
use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods};


pub mod siteindex;
pub mod sitepage;
pub mod pagerecord;
pub mod linkindex;


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
        Err(_) => Err("Error connecting to DB for existing index check".to_string()),
        Ok(exists) => match exists {
            false => Err(format!("No Site exists in DB with given ID: {:?}", indexid).to_string()),
            // Checking for both existing records for the same id AND texturl can't be done easily in Diesel,
            // so it is done serially => get all records for given id, then iterate over that list for given url.
            // this is most likely quicker than getting all records for a URL.
            true => match sitepage::db_get_records(connection, indexid) {
                Err(_) => Err("Error getting records from DB for site index check".to_string()),
                Ok(list) => match list.iter().any(|record| record.texturl.as_ref().unwrap() == texturl) {
                    true => Err("Site Page for this URL already exists in DB".to_string()),
                    false => {sitepage::db_add_site_page(connection, indexid, texturl); Ok(())}

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
                    Some(_) => Ok(()),
                    
                    // Page record wasn't added successfully
                    None => Err("Page record couldn't be added to DB succesfully.".to_string())
                }
        }

    }

}

pub fn create_link_index(connection: &PgConnection, fromid: i32, toid: i32) -> Result<(), String> {
    // Check that the toid is valid in the pages table.
    match sitepage::db_check_id_exists(connection, toid){
        Err(_) => Err("Error connecting to DB for existing toIndex check".to_string()),
        Ok(exists) => match exists {
            false => Err(format!("No Page exists in DB with given id: {:?}", toid).to_string()),
            // Check that fromid is valid in the pages table.
            true => match sitepage::db_check_id_exists(connection, fromid) {
                Err(_) => Err("Error connecting to DB for existing fromIndex check".to_string()),
                Ok(exists) => match exists {
                    false => Err(format!("No Page exists in DB with given id: {:?}", fromid).to_string()),
                    // Get list of all 'from'
                    true => match linkindex::db_get_records_from(connection, fromid) {
                        Err(_) => Err("Error connecting to DB for collecting existing 'from' list.".to_string()),
                        // If a valid list of 'from' addresses is returned, check for the 'to' destination in that list.
                        Ok(list) => match list.iter().any(|record| record.toid == toid) {
                            true => Err("This link pair already exists in DB".to_string()),
                            false => {linkindex::db_add_link_index(connection, fromid, toid); Ok(())}

                        }

                    }
                }

            }
        }
    }
    // match sitepages::db_get_records(connection, toid) {
    //     Err(e) => Err("Error connecting to DB for existing index check".to_string()),
    //     // Check that the destination id exists.
    //     Ok(list) =
    // }   
    // match linkindex::db_add_link_index(connection, fromid, toid) {
    //     None => Err("Link index couldn't be added to DB successfully".to_string()),
    //     Some(val) => Ok(())
    // }

}