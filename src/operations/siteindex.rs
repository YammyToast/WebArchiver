#[macro_use]
extern crate diesel;
use crate::model::{SiteIndex, AddSiteIndex};
use diesel::PgConnection;

// Add siteindex function.
// Parameters:
// connection: valid database connection.
// name: name property of the index.
// domain: domain property of the index.
pub fn addSiteIndex<'a>(connection: &PgConnection, _name: &'a str, _domain: &'a str) -> SiteIndex {
    use crate::schema::siteindexs;

    let new_index = AddSiteIndex {
        name: _name,
        domain: _domain,
    };

    diesel::insert_into(siteindexs).values(&new_index).get_results(conn).expect("Error creating new post");

}