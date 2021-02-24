table! {
    actix_users (uuid) {
        uuid -> Uuid,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
    }
}

table! {
    contacts (contact_id) {
        contact_id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
    }
}

table! {
    events (eventid) {
        eventid -> Uuid,
        title -> Varchar,
        date -> Varchar,
        duration -> Int4,
        creator -> Nullable<Uuid>,
    }
}

table! {
    person (id) {
        id -> Int4,
        name -> Varchar,
        data -> Nullable<Bytea>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    requests (requestid) {
        requestid -> Uuid,
        requester -> Nullable<Uuid>,
        requested -> Nullable<Uuid>,
    }
}

table! {
    users (uuid) {
        uuid -> Uuid,
        username -> Nullable<Varchar>,
        password -> Varchar,
    }
}

joinable!(events -> users (creator));

allow_tables_to_appear_in_same_query!(
    actix_users,
    contacts,
    events,
    person,
    posts,
    requests,
    users,
);
