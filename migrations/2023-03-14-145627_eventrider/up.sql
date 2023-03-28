-- Your SQL goes here

/*
pub struct EventRider{
    pub e_id: String,
    pub r_id: String,
    pub er_type: String,
    pub er_specialization: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
*/


create table EventRider(
    e_id character varying(255) not null,
    r_id character varying(255) not null,
    er_type character varying(255) not null,
    er_specialization character varying(255) not null,
    created_at timestamp without time zone not null,
    updated_at timestamp without time zone,
    primary key (e_id, r_id),
    foreign key (e_id) references Events(e_id),
    foreign key (r_id) references Riders(r_id)
);