// @generated automatically by Diesel CLI.

diesel::table! {
    businesses (id) {
        id -> Int4,
        uuid -> Varchar,
        business -> Varchar,
        business_type -> Varchar,
        location -> Varchar,
        selected_amenities -> Array<Nullable<Text>>,
        images -> Array<Nullable<Text>>,
        business_name -> Varchar,
        telephone_number -> Varchar,
        business_description -> Varchar,
        days_of_operation -> Array<Nullable<Text>>,
        opening_hours -> Varchar,
        closing_hours -> Varchar,
        county -> Varchar,
        town -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        uuid -> Varchar,
        name -> Varchar,
        businessid -> Varchar,
        review -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        uuid -> Varchar,
        name -> Varchar,
        email -> Varchar,
        image -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    businesses,
    reviews,
    users,
);
