// @generated automatically by Diesel CLI.

diesel::table! {
    crates (id) {
        id -> Int4,
        rustacean_id -> Int4,
        code -> Varchar,
        name -> Varchar,
        version -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        _types -> Nullable<Array<Nullable<Text>>>,
        organisation_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    organisation_events (id) {
        id -> Int4,
        organisation_id -> Int4,
        event_id -> Int4,
    }
}

diesel::table! {
    organisations (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        code -> Varchar,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    rustaceans (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::joinable!(crates -> rustaceans (rustacean_id));
diesel::joinable!(events -> organisations (organisation_id));
diesel::joinable!(organisation_events -> events (event_id));
diesel::joinable!(organisation_events -> organisations (organisation_id));
diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    crates,
    events,
    organisation_events,
    organisations,
    roles,
    rustaceans,
    users,
    users_roles,
);
