use crate::schema::*;

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(linkid)]
pub struct Linkindex {
    pub linkid: i32,
    pub fromid: i32,
    pub toid: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(siteid)]
pub struct Siteindex {
    pub siteid: i32,
    pub name: Option<String>,
    pub domain: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(pageid)]
pub struct Sitepage {
    pub pageid: i32,
    pub siteid: i32,
    pub texturl: Option<String>,
}

