-- Your SQL goes here

/*
pub struct Helmet {
    pub id: Option<String>,
    pub brand: String,
    pub model: String,
    pub htype: String,
    pub size: String,
    pub price: f64,
    pub sold: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
*/

create table Helmets (
    h_id character varying(255) primary key,
    brand character varying(255) not null,
    model character varying(255) not null,
    h_type character varying(255) not null,
    size character varying(255) not null,
    price float not null,
    sold boolean not null default false,
    created_at timestamp without time zone not null,
    updated_at timestamp without time zone
);