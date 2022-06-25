// Diesel Imports
use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods};

// Data Models Imports
use crate::model::{Siteindex, AddSiteIndex};
use crate::schema::siteindexs::dsl::*;



// Add siteindex function.
// Parameters:
// connection: diesel database connection struct.
// name: name property of the index.
// domain: domain property of the index.
pub fn db_add_site_index<'a>(connection: &PgConnection, _name: &'a str, _domain: &'a str) -> Option<Siteindex> {
    use crate::schema::siteindexs;

    // Create insert struct (defined in schema)
    let new_index = AddSiteIndex {
        name: _name,
        domain: _domain,
    };

    // Perform insert.
    // TODO: Improve error handling.
    match diesel::insert_into(siteindexs::table).values(&new_index).get_result(connection) {
        Ok(val) => Some(val),
        _ => None

    }
}

pub fn db_get_records<'a>(connection: &PgConnection, check_query: &'a str) -> Result<Vec<Siteindex>, ()> {
    match siteindexs.filter(name.eq(check_query)).load::<Siteindex>(connection){
        Err(_) => Err(()),
        Ok(list) => Ok(list)
    } 

}

pub fn db_check_id_exists(connection: &PgConnection, check_id: i32) -> Result<bool, ()> {
    match siteindexs.filter(siteid.eq(check_id)).load::<Siteindex>(connection){
        Err(_) => Err(()),
        Ok(list) => match list.len() {
            0 => Ok(false),
            1.. => Ok(true),  
            _ => Err(())
        }

    }

}


// pub fn db_remove_site_index_by_name<'a>(connection: &PgConnection, name_query: &'a str) {


// }

// pub fn db_remove_site_index_by_domain<'a>(connection: &PgConnection, domain_query: &'a str) {


// }