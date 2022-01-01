table! {
    access_tokens (id) {
        id -> Uuid,
        client_id -> Uuid,
        attributes -> Nullable<Jsonb>,
        created_datetime -> Timestamp,
        modified_datetime -> Timestamp,
    }
}

table! {
    clients (id) {
        id -> Uuid,
        client_name -> Varchar,
        audience -> Jsonb,
        jwks -> Nullable<Jsonb>,
        created_datetime -> Timestamp,
        modified_datetime -> Timestamp,
    }
}

table! {
    refresh_tokens (id) {
        id -> Uuid,
        client_id -> Uuid,
        created_datetime -> Timestamp,
        modified_datetime -> Timestamp,
    }
}

joinable!(access_tokens -> clients (client_id));
joinable!(refresh_tokens -> clients (client_id));

allow_tables_to_appear_in_same_query!(access_tokens, clients, refresh_tokens,);
