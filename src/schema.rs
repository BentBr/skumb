// @generated automatically by Diesel CLI.

diesel::table! {
    chat_messages (id) {
        id -> Int4,
        chat_id -> Int4,
        creator_id -> Int4,
        uuid -> Uuid,
        text -> Varchar,
        creation_date -> Timestamp,
    }
}

diesel::table! {
    chats (id) {
        id -> Int4,
        creator_id -> Int4,
        uuid -> Uuid,
        name -> Varchar,
        share_uri -> Nullable<Varchar>,
        creation_date -> Timestamp,
        modification_date -> Nullable<Timestamp>,
        deletion_date -> Nullable<Timestamp>,
    }
}

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

diesel::joinable!(chat_messages -> chats (chat_id));
diesel::joinable!(chat_messages -> users (creator_id));
diesel::joinable!(chats -> users (creator_id));

diesel::allow_tables_to_appear_in_same_query!(chat_messages, chats, users,);
