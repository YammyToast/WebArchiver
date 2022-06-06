table! {
    linkindexs (linkid) {
        linkid -> Int4,
        fromid -> Int4,
        toid -> Int4,
    }
}

table! {
    siteindexs (siteid) {
        siteid -> Int4,
        name -> Nullable<Varchar>,
        domain -> Nullable<Varchar>,
    }
}

table! {
    sitepages (pageid) {
        pageid -> Int4,
        siteid -> Int4,
        texturl -> Nullable<Text>,
    }
}

joinable!(sitepages -> siteindexs (siteid));

allow_tables_to_appear_in_same_query!(
    linkindexs,
    siteindexs,
    sitepages,
);
