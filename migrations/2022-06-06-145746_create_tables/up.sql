-- Your SQL goes here
CREATE TABLE siteindex
(
    SiteID SERIAL PRIMARY KEY,
    Name varchar(255),
    Domain varchar(255) 
);

CREATE TABLE sitepages
(
    PageID SERIAL PRIMARY KEY,
    SiteID SERIAL REFERENCES siteindex(SiteID),
    TextURL text
);

CREATE TABLE linkindex
(
    LinkID SERIAL PRIMARY KEY,
    FromID SERIAL REFERENCES sitepages(PageID),
    ToID SERIAL REFERENCES sitepages(PageID)
);

CREATE TABLE pagerecords (
    RecordID SERIAL PRIMARY KEY,
    PageID SERIAL REFERENCES sitepages(PageID),
    Date date,
    VaultID UUID
);