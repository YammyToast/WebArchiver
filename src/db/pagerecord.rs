// Diesel Imports
use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods};

// Data Models Imports
use crate::model::{Pagerecord, AddPageRecords};
use crate::schema::pagerecords::dsl::*;

// Generator Imports
use std::time::SystemTime;
use uuid::Uuid;



pub fn db_add_page_record(connection: &PgConnection, _pageid : i32) -> Option<Pagerecord> {

    use crate::schema::pagerecords;

    // Create insert struct (defined in schema)
    let new_record = AddPageRecords {
        pageid: _pageid,
        date: SystemTime::now(),
        vaultid: Uuid::new_v4()
    };

    // Perform insert.
    // TODO: Improve error handling.
    let result = diesel::insert_into(pagerecords::table)
    .values(&new_record)
    .get_result(connection);

    match result {
        Ok(val) => Some(val),
        _ => None

    }

}