
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::schema::{bikes, helmets, riders,events,eventrider};
#[allow(non_snake_case)]
#[derive(Queryable,Insertable,Clone,Serialize, Deserialize, Debug,AsChangeset)]
#[diesel(table_name = bikes)]
pub struct Bike {
    pub b_id: String,
    pub brand: String,
    pub model: String,
    pub wheelsize: f64,
    pub size: String,
    pub price: f64,
    pub sold: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,

}
#[allow(non_snake_case)]
#[derive(Queryable,Insertable,Clone,Serialize, Deserialize, Debug,AsChangeset)]
#[diesel(table_name = helmets)]
pub struct Helmet {
    pub h_id: String,
    pub brand: String,
    pub model: String,
    pub h_type: String,
    pub size: String,
    pub price: f64,
    pub sold: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[allow(non_snake_case)]
#[derive(Queryable, Insertable, Clone, Serialize, Deserialize, Debug,AsChangeset)]
#[diesel(table_name = riders)]
pub struct Rider {
    pub r_id: String,
    pub helmet_id: String,
    pub bike_id: String,
    pub r_name: String,
    pub height: f64,
    pub r_weight: f64,
    pub specialization: String,
    pub email: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize,Queryable, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = events)]
pub struct Event{
    pub e_id: String,
    pub e_name: String,
    pub e_date: NaiveDateTime,
    pub specialization: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize,Queryable, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = eventrider)]
pub struct EventRider{
    pub e_id: String,
    pub r_id: String,
    pub er_type: String,
    pub er_specialization: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
pub struct AppState {
    pub bikes_db: Arc<Mutex<Vec<Bike>>>,
    pub helmets_db: Arc<Mutex<Vec<Helmet>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            bikes_db: Arc::new(Mutex::new(Vec::new())),
            helmets_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UpdateBike {
    pub brand: Option<String>,
    pub model: Option<String>,
    pub wheelsize: Option<f64>,
    pub size: Option<String>,
    pub price: Option<f64>,
    pub sold: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UpdateHelmet {
    pub brand: Option<String>,
    pub model: Option<String>,
    pub htype: Option<String>,
    pub size: Option<String>,
    pub price: Option<f64>,
    pub sold: Option<bool>,
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UpdateRider {
    pub helmet_id: Option<String>,
    pub bike_id: Option<String>,
    pub r_name: Option<String>,
    pub height: Option<f64>,
    pub r_weight: Option<f64>,
    pub specialization: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UpdateEvent {
    pub e_name: Option<String>,
    pub e_date: Option<NaiveDateTime>,
    pub specialization: Option<String>,
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UpdateEventRider {
    pub er_type: Option<String>,
    pub er_specialization: Option<String>,
}