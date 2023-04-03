use crate::{
    model::{Event,Rider,AppState, UpdateEvent,EventRider},
    response::{EventListResponse, EventData,SingleEventWithRidersResponse,GenericResponse},
    handlers::eventriderhandler::delete_eventrider_dependencies,
    db::establish_connection
};

use diesel::prelude::*;
use chrono::prelude::*;
use uuid::Uuid;

use rocket::{
    options, delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State
};

use crate::diesel::RunQueryDsl;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};


#[openapi(tag = "Events")]
#[get("/events?<page>&<limit>")]
pub async fn events_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<EventListResponse>, Status> {
    
    use crate::schema::events::dsl::*;
    let connection = &mut establish_connection();
    let vec = events
        .load::<Event>(connection)
        .expect("Error loading events");
    
    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;
    let good_events: Vec<Event> = vec.clone().into_iter().skip(offset).take(limit).collect();

    let response_json = EventListResponse {
        status: "success".to_string(),
        results: good_events.len(),
        events:good_events
    };

    Ok(Json(response_json))
}

#[openapi(tag = "Events")]
#[post("/events", data = "<body>")]
pub async fn create_event_handler(
    mut body: Json<Event>,
    data: &State<AppState>,
) -> Result<Json<SingleEventWithRidersResponse>, Custom<Json<GenericResponse>>>{
    
    use crate::schema::events::dsl::*;
    let connection = &mut establish_connection();
    let vec = events
        .load::<Event>(connection)
        .expect("Error loading events");
    for event in vec.iter() {
        if event.e_name == body.e_name && event.e_date==event.e_date{
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Event already exists DING DONG".to_string(),
            };
            return Err(Custom(Status::BadRequest, Json(response_json)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now().naive_utc();
    body.created_at = datetime;
    body.updated_at = Some(datetime);
    let new_event =Event{
        e_id: uuid_id.to_string(),
        e_name: body.e_name.clone(),
        specialization: body.specialization.clone(),
        e_date: body.e_date.clone(),
        created_at:body.created_at.clone(),
        updated_at: Some(datetime.clone()),
    };


    let event = new_event.to_owned(); 
    let event_for_db = event.clone();

    let connection = &mut establish_connection();
    diesel::insert_into(events)
    .values(&event_for_db)
    .execute(connection)
    .expect("Error saving new event PAOAO");
    let rider_list = get_riders_for_event(event.e_id.clone());
    let json_response =SingleEventWithRidersResponse{
        status: "success".to_string(),
        event: EventData {
            event: event,
        },
        riders:rider_list
    };

    Ok(Json(json_response))
}

pub fn get_riders_for_event(eventid: String) -> Vec<Rider> {
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let event_id_clone = eventid.clone();
    let result = riders
        .find(event_id_clone)
        .load::<Rider>(connection)
        .expect("Error loading riders");
    result.clone()
}

#[openapi(tag = "Events")]
#[get("/events/<event_id>")]
pub async fn get_event_handler(
    event_id: String,
    data: &State<AppState>,
) -> Result<Json<SingleEventWithRidersResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::events::dsl::*;
    let connection = &mut establish_connection();
    let event_id_clone = event_id.clone();
    let result = events
        .find(event_id_clone)
        .first::<Event>(connection)
        .ok();

    match result {
        Some(event) => {
            let rider_list = get_riders_for_event(event.e_id.clone());
            let json_response = SingleEventWithRidersResponse {
                status: "success".to_string(),
                event: EventData {
                    event: event.to_owned(),
                },
                riders:rider_list
            };
            return Ok(Json(json_response));
        }
        None => {
            let error = GenericResponse {
                status: "error".to_string(),
                message: "Event not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(error)));
        }
    }

}

#[openapi(tag = "Events")]
#[patch("/events/<event_id>", data = "<body>")]
pub async fn update_event_handler(
    event_id: String,
    body: Json<UpdateEvent>,
    data: &State<AppState>,
) -> Result<Json<SingleEventWithRidersResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::events::dsl::*;
    let connection = &mut establish_connection();
    let event_id_clone = event_id.clone();
    let result = events
        .find(event_id_clone)
        .first::<Event>(connection)
        .ok();
    
    match result {
        None => {
            let error = GenericResponse {
                status: "error".to_string(),
                message: "Event not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(error)));
        }
        Some(old_event) => {
            let datetime = Utc::now().naive_utc();
            let e_e_name = body.e_name.to_owned().unwrap_or(old_event.e_name.to_owned());
            let e_e_date = body.e_date.to_owned().unwrap_or(old_event.e_date.to_owned());
            let e_specialization = body.specialization.to_owned().unwrap_or(old_event.specialization.to_owned());
            
            let payload = Event {
            e_id: old_event.e_id.to_owned(),
            e_name: if !e_e_name.is_empty() { 
                e_e_name 
            } else {
                old_event.e_name.to_owned()
            },
            e_date: e_e_date,
            specialization: if !e_specialization.is_empty() { 
                e_specialization 
            } else {
                old_event.specialization.to_owned()
            },created_at: old_event.created_at.to_owned(),
            updated_at: Some(datetime),
            };

            let connection = &mut establish_connection();
            diesel::update(events.find(old_event.e_id.clone()))
            .set(&payload)
            .execute(connection)
            .expect("Error updating event");
            
            let rider_list = get_riders_for_event(payload.e_id.clone());

            let json_response = SingleEventWithRidersResponse {
                status: "success".to_string(),
                event: EventData {
                    event: payload.clone(),
                },
                riders:rider_list
        };
        return Ok(Json(json_response));
        }
        
        
    }
}

pub fn delete_event_dependencies(eventid: String) -> Result<usize, diesel::result::Error>{
    use crate::schema::eventrider::dsl::*;
    let connection = &mut establish_connection();
    let result = eventrider
        .filter(e_id.eq(eventid.clone()))
        .load::<EventRider>(connection)
        .expect( "Error loading riders");

    for event_rider in result{
       match  delete_eventrider_dependencies(event_rider.e_id.clone(), event_rider.r_id.clone()){
           Ok(_) => {
           }
           Err(_) => {
                return Err(diesel::result::Error::NotFound);
           }
        }
    }

    let connection = &mut establish_connection();
    let rider_id_clone = eventid.clone();
    diesel::delete(eventrider.filter(e_id.eq(rider_id_clone))).execute(connection)
}

#[openapi(tag = "Events")]
#[delete("/event/<event_id>")]
pub async fn delete_event_handler(
    event_id: String,
    data: &State<AppState>,
) -> Result<Json<GenericResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::events::dsl::*;
    let connection = &mut establish_connection();
    let event_id_clone = event_id.clone();

    match diesel::delete(events.find(event_id_clone))
        .execute(connection){
        Ok(_) => {
            let response_json = GenericResponse {
                status: "success".to_string(),
                message: "Event deleted".to_string(),
            };

            return Ok(Json(response_json));
        }
        Err(_) => {
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Event not found".to_string(),
            };

            return Err(Custom(Status::NotFound, Json(response_json)));
        }
    }
}
