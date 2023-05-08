use crate::model::Bike;
use crate::model::Helmet;
use crate::model::Rider;
use crate::model::Event;
use crate::model::EventRider;
use serde::Serialize;
use serde::Deserialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

#[derive(Serialize, JsonSchema)]
pub struct GenericResponse{
    pub status: String,
    pub message: String,
}

//HERE THE BIKE RESPONSES STANDs

#[derive(Serialize, Debug, JsonSchema)]
pub struct BikeData {
    pub bike: Bike,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct SingleBikeResponse {
    pub status: String,
    pub bike: BikeData,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct SingleBikeWRidersResponse {
    pub status: String,
    pub bike: BikeData,
    pub riders: Vec<Rider>,
}

#[derive(Serialize, Debug, JsonSchema)]
pub struct BikeListResponse {
    pub status: String,
    pub results: usize,
    pub bikes: Vec<Bike>,
    pub counts: Vec<usize>,
}

//HERE THE HELMET RESPONSES STAND

#[derive(Serialize,Debug, JsonSchema)]
pub struct HelmetData {
    pub helmet: Helmet,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct SingleHelmetResponse {
    pub status: String,
    pub helmet: HelmetData,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct SingleHelmetWRidersResponse {
    pub status: String,
    pub helmet: HelmetData,
    pub riders: Vec<Rider>,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct HelmetListResponse {
    pub status: String,
    pub results: usize,
    pub helmets: Vec<Helmet>,
    pub counts: Vec<usize>,
}

#[derive(Serialize,Debug,Deserialize, JsonSchema)]
pub struct HelmetStat {
    pub helmet: Helmet,
    pub no_riders: usize,
}

#[derive(Serialize,Debug,Deserialize, JsonSchema)]
pub struct HelmetStatListResponse{
    pub status: String,
    pub results: usize,
    pub helmets: Vec<HelmetStat>,
}


//HERE THE RIDER RESPONSES STAND

#[derive(Serialize,Debug, JsonSchema)]
pub struct RiderData {
    pub rider: Rider,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct SingleRiderWithGearResponse {
    pub status: String,
    pub rider: RiderData,
    pub bike: Bike,
    pub helmet: Helmet,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct SingleRiderWithEventsResponse {
    pub status: String,
    pub rider: RiderData,
    pub events: Vec<Event>,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct RiderListResponse {
    pub status: String,
    pub results: usize,
    pub riders: Vec<Rider>,
    pub counts: Vec<usize>,
}
#[derive(Serialize,Debug,Deserialize, JsonSchema)]
pub struct RiderStat {
    pub rider: Rider,
    pub no_events: usize,
}

#[derive(Serialize,Debug,Deserialize, JsonSchema)]
pub struct RiderStatListResponse{
    pub status: String,
    pub results: usize,
    pub riders: Vec<RiderStat>,
}


//HERE THE EVENT RESPONSES STAND

#[derive(Serialize,Debug, JsonSchema)]
pub struct EventData {
    pub event: Event,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct SingleEventWithRidersResponse {
    pub status: String,
    pub event: EventData,
    pub riders: Vec<Rider>,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct EventListResponse {
    pub status: String,
    pub results: usize,
    pub events: Vec<Event>,
}



//HERE THE EVENTRIDER RESPONSES STAND

#[derive(Serialize,Debug, JsonSchema)]
pub struct EventRiderData {
    pub eventrider: EventRider,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct SingleEventRiderResponse {
    pub status: String,
    pub eventrider: EventRiderData,
}

#[derive(Serialize,Debug, JsonSchema)]
pub struct EventRiderListResponse {
    pub status: String,
    pub results: usize,
    pub eventriders: Vec<EventRider>,
}
