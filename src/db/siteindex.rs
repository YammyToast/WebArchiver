use crate::diesel::RunQueryDsl;
use crate::model::{Siteindex, AddSiteIndex};
use diesel::PgConnection;

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
    let result = diesel::insert_into(siteindexs::table)
    .values(&new_index)
    .get_result(connection);

    match result {
        Ok(val) => Some(val),
        _ => None

    }
}

pub fn db_remove_site_index_by_name<'a>(connection: &PgConnection, name_query: &'a str) {


}

pub fn db_remove_site_index_by_domain<'a>(connection: &PgConnection, domain_query: &'a str) {


}