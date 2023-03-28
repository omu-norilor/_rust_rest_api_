-- Your SQL goes here

/*
pub struct Rider {
    pub id: Option<String>,
    pub helmet_id: String,
    pub bike_id: String,
    pub name: String,
    pub height: f64,
    pub weight: f64,
    pub specialization: String,
    pub email: String,
    pub phone: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
*/

create table Riders (
    r_id character varying(255) primary key,
    helmet_id character varying(255) not null,
    bike_id character varying(255) not null,
    r_name varchar(255) not null,
    height float not null,
    r_weight float not null,
    specialization varchar(255) not null,
    email varchar(255) not null,
    phone varchar(255) not null,
    created_at timestamp without time zone not null,
    updated_at timestamp without time zone,
    foreign key (helmet_id) references Helmets(h_id),
    foreign key (bike_id) references Bikes(b_id)
);