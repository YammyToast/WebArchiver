use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods};

// Data Models Imports
use crate::model::{Sitepage, AddSitePages};
use crate::schema::sitepages::dsl::*;

pub fn db_add_site_page<'a>(connection: &PgConnection, _siteid: i32, _texturl: &'a str) -> Option<Sitepage> {
    use crate::schema::sitepages;

    let new_page = AddSitePages {
        siteid: _siteid,
        texturl: _texturl
    };

    


    
    match diesel::insert_into(sitepages::table).values(&new_page).get_result(connection){
        Ok(val) => Some(val),
        _ => None

    }
}

pub fn db_get_records(connection: &PgConnection, _siteid: i32) -> Result<Vec<Sitepage>, ()> {
    match sitepages.filter(siteid.eq(_siteid)).load::<Sitepage>(connection) {
        Err(_) => Err(()),
        Ok(list) => Ok(list)
    }
}


pub fn db_check_id_exists(connection: &PgConnection, check_id: i32) -> Result<bool, ()> {
    match sitepages.filter(pageid.eq(check_id)).load::<Sitepage>(connection){
        Err(_) => Err(()),
        Ok(list) => match list.len() {
            0 => Ok(false),
            1.. => Ok(true),  
            _ => Err(())
        }

    }

}