-- Your SQL goes here

/*pub struct Event{
    pub e_id: String,
    pub e_name: String,
    pub date: NaiveDateTime,
    pub specialization: String,
}*/

create table Events(
    e_id character varying(255) primary key,
    e_name character varying(255) not null,
    e_date timestamp without time zone not null,
    specialization character varying(255) not null,
    created_at timestamp without time zone not null,
    updated_at timestamp without time zone
);