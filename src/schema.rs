// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        uuid -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        salutation -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        creation_date -> Timestamp,
        modification_date -> Nullable<Timestamp>,
        deletion_date -> Nullable<Timestamp>,
    }
}
