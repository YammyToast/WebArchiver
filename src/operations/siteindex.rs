use crate::diesel::RunQueryDsl;
use crate::model::{Siteindex, AddSiteIndex};
use diesel::PgConnection;

// Add siteindex function.
// Parameters:
// connection: valid database connection.
// name: name property of the index.
// domain: domain property of the index.
pub fn addSiteIndex<'a>(connection: &PgConnection, _name: &'a str, _domain: &'a str) -> Siteindex {
    use crate::schema::siteindexs;

    let new_index = AddSiteIndex {
        name: _name,
        domain: _domain,
    };


    diesel::insert_into(siteindexs::table)
    .values(&new_index)
    .get_result(connection)
    .expect("Error creating new post")


}