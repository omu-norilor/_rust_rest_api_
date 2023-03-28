// @generated automatically by Diesel CLI.

diesel::table! {
    bikes (b_id) {
        b_id -> Varchar,
        brand -> Varchar,
        model -> Varchar,
        wheelsize -> Float8,
        size -> Varchar,
        price -> Float8,
        sold -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    eventrider (e_id, r_id) {
        e_id -> Varchar,
        r_id -> Varchar,
        er_type -> Varchar,
        er_specialization -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    events (e_id) {
        e_id -> Varchar,
        e_name -> Varchar,
        e_date -> Timestamp,
        specialization -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    helmets (h_id) {
        h_id -> Varchar,
        brand -> Varchar,
        model -> Varchar,
        h_type -> Varchar,
        size -> Varchar,
        price -> Float8,
        sold -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    riders (r_id) {
        r_id -> Varchar,
        helmet_id -> Varchar,
        bike_id -> Varchar,
        r_name -> Varchar,
        height -> Float8,
        r_weight -> Float8,
        specialization -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(eventrider -> events (e_id));
diesel::joinable!(eventrider -> riders (r_id));
diesel::joinable!(riders -> bikes (bike_id));
diesel::joinable!(riders -> helmets (helmet_id));

diesel::allow_tables_to_appear_in_same_query!(
    bikes,
    eventrider,
    events,
    helmets,
    riders,
);
