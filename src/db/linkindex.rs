// Diesel Imports
use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods};

// Data Models Imports
use crate::model::{Linkindex, AddLinkIndex};
use crate::schema::linkindexs::dsl::*;



pub fn db_add_link_index(connection: &PgConnection, _fromid: i32, _toid: i32) -> Option<Linkindex>{
    use crate::schema::linkindexs;

    let new_link = AddLinkIndex {
        fromid: _fromid,
        toid: _toid
    };

    match diesel::insert_into(linkindexs::table).values(&new_link).get_result(connection){
        Ok(val) => Some(val),
        _ => None

    }
}

pub fn db_get_records_from(connection: &PgConnection, check_id: i32) -> Result<Vec<Linkindex>, ()> {
    match linkindexs.filter(fromid.eq(check_id)).load::<Linkindex>(connection){
        Err(_) => Err(()),
        Ok(list) => Ok(list)
    } 
}

pub fn db_get_records_to(connection: &PgConnection, check_id: i32) -> Result<Vec<Linkindex>, ()> {
    match linkindexs.filter(toid.eq(check_id)).load::<Linkindex>(connection){
        Err(_) => Err(()),
        Ok(list) => Ok(list)
    } 
}

