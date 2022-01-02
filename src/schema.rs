table! {
    access_tokens (id) {
        id -> Uuid,
        client_id -> Uuid,
        active -> Bool,
        attributes -> Nullable<Jsonb>,
        created_datetime -> Timestamp,
        modified_datetime -> Timestamp,
    }
}

table! {
    clients (id) {
        id -> Uuid,
        tenant_id -> Uuid,
        client_name -> Varchar,
        created_datetime -> Timestamp,
        modified_datetime -> Timestamp,
    }
}

table! {
    refresh_tokens (id) {
        id -> Uuid,
        client_id -> Uuid,
        active -> Bool,
        created_datetime -> Timestamp,
        modified_datetime -> Timestamp,
    }
}

table! {
    tenants (id) {
        id -> Uuid,
        tenant_name -> Varchar,
        issuer -> Varchar,
        access_token_ttl -> Int4,
        refresh_token_ttl -> Int4,
        created_datetime -> Timestamp,
        modified_datetime -> Timestamp,
    }
}

joinable!(access_tokens -> clients (client_id));
joinable!(clients -> tenants (tenant_id));
joinable!(refresh_tokens -> clients (client_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    clients,
    refresh_tokens,
    tenants,
);
