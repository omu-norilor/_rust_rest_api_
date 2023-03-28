use crate::model::Bike;
use crate::model::Helmet;
use crate::model::Rider;
use crate::model::Event;
use crate::model::EventRider;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize)]
pub struct GenericResponse{
    pub status: String,
    pub message: String,
}

//HERE THE BIKE RESPONSES STANDs

#[derive(Serialize,Debug)]
pub struct BikeData {
    pub bike: Bike,
}

#[derive(Serialize,Debug)]
pub struct SingleBikeResponse {
    pub status: String,
    pub bike: BikeData,
}

#[derive(Serialize,Debug)]
pub struct SingleBikeWRidersResponse {
    pub status: String,
    pub bike: BikeData,
    pub riders: Vec<Rider>,
}

#[derive(Serialize,Debug)]
pub struct BikeListResponse {
    pub status: String,
    pub results: usize,
    pub bikes: Vec<Bike>,
}

//HERE THE HELMET RESPONSES STAND

#[derive(Serialize,Debug)]
pub struct HelmetData {
    pub helmet: Helmet,
}

#[derive(Serialize,Debug)]
pub struct SingleHelmetResponse {
    pub status: String,
    pub helmet: HelmetData,
}

#[derive(Serialize,Debug)]
pub struct HelmetListResponse {
    pub status: String,
    pub results: usize,
    pub helmets: Vec<Helmet>,
}

#[derive(Serialize,Debug,Deserialize)]
pub struct HelmetStat {
    pub helmet: Helmet,
    pub no_riders: usize,
}

#[derive(Serialize,Debug,Deserialize)]
pub struct HelmetStatListResponse{
    pub status: String,
    pub helmets: Vec<HelmetStat>,
}


//HERE THE RIDER RESPONSES STAND

#[derive(Serialize,Debug)]
pub struct RiderData {
    pub rider: Rider,
}

#[derive(Serialize,Debug)]
pub struct SingleRiderWithGearResponse {
    pub status: String,
    pub rider: RiderData,
    pub bike: Bike,
    pub helmet: Helmet,
}

#[derive(Serialize,Debug)]
pub struct SingleRiderWithEventsResponse {
    pub status: String,
    pub rider: RiderData,
    pub events: Vec<Event>,
}

#[derive(Serialize,Debug)]
pub struct RiderListResponse {
    pub status: String,
    pub results: usize,
    pub riders: Vec<Rider>,
}
#[derive(Serialize,Debug,Deserialize)]
pub struct RiderStat {
    pub rider: Rider,
    pub no_events: usize,
}

#[derive(Serialize,Debug,Deserialize)]
pub struct RiderStatListResponse{
    pub status: String,
    pub riders: Vec<RiderStat>,
}


//HERE THE EVENT RESPONSES STAND

#[derive(Serialize,Debug)]
pub struct EventData {
    pub event: Event,
}

#[derive(Serialize,Debug)]
pub struct SingleEventWithRidersResponse {
    pub status: String,
    pub event: EventData,
    pub riders: Vec<Rider>,
}

#[derive(Serialize,Debug)]
pub struct EventListResponse {
    pub status: String,
    pub results: usize,
    pub events: Vec<Event>,
}



//HERE THE EVENTRIDER RESPONSES STAND

#[derive(Serialize,Debug)]
pub struct EventRiderData {
    pub eventrider: EventRider,
}

#[derive(Serialize,Debug)]
pub struct SingleEventRiderResponse {
    pub status: String,
    pub eventrider: EventRiderData,
}

#[derive(Serialize,Debug)]
pub struct EventRiderListResponse {
    pub status: String,
    pub results: usize,
    pub eventriders: Vec<EventRider>,
}
