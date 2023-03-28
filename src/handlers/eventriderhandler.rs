use crate::{
    model::{EventRider,AppState,UpdateEventRider},
    response::{ EventRiderData,SingleEventRiderResponse,EventRiderListResponse,GenericResponse},
    db::establish_connection,
    
};

use diesel::prelude::*;
use chrono::prelude::*;
use uuid::Uuid;

use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State
};

use crate::diesel::RunQueryDsl;
use crate::schema::eventrider::dsl::*;


#[get("/eventrider/getall?<page>&<limit>")]
pub async fn eventrider_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<EventRiderListResponse>, Status> {
    
    use crate::schema::eventrider::dsl::*;
    let connection = &mut establish_connection();
    let vec = eventrider
        .load::<EventRider>(connection)
        .expect("Error loading eventrider");
    
    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;
    let good_eventriders: Vec<EventRider> = vec.clone().into_iter().skip(offset).take(limit).collect();

    let response_json = EventRiderListResponse {
        status: "success".to_string(),
        results: good_eventriders.len(),
        eventriders:good_eventriders
    };

    Ok(Json(response_json))
}


#[post("/eventrider", data = "<body>")]
pub async fn create_eventrider_handler(
    mut body: Json<EventRider>,
    data: &State<AppState>,
) -> Result<Json<SingleEventRiderResponse>, Custom<Json<GenericResponse>>>{
    
    use crate::schema::eventrider::dsl::*;
    let connection = &mut establish_connection();
    let vec = eventrider
        .load::<EventRider>(connection)
        .expect("Error loading eventrider");
    for eventrider_x in vec.iter() {
        if eventrider_x.e_id == body.e_id && eventrider_x.r_id==eventrider_x.r_id{
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "EventRider already exists DING DONG".to_string(),
            };
            return Err(Custom(Status::BadRequest, Json(response_json)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now().naive_utc();
    body.created_at = datetime;
    body.updated_at = Some(datetime);
    let new_eventrider =EventRider{
        e_id: body.e_id.clone(),
        r_id: body.r_id.clone(),
        er_type: body.er_type.clone(),
        er_specialization: body.er_specialization.clone(),
        created_at:body.created_at.clone(),
        updated_at: Some(datetime.clone()),
    };


    let eventrider_x = new_eventrider.to_owned(); 
    let eventrider_for_db = eventrider_x.clone();

    let connection = &mut establish_connection();
    diesel::insert_into(eventrider)
    .values(&eventrider_for_db)
    .execute(connection)
    .expect("Error saving new eventrider PAOAO");
    let json_response =SingleEventRiderResponse{
        status: "success".to_string(),
        eventrider: EventRiderData {
            eventrider: eventrider_x,
        }
    };

    Ok(Json(json_response))
}

#[get("/eventrider/getbyids?<event_id>&<rider_id>")]
pub async fn get_eventrider_handler(
    event_id: String,
    rider_id: String,
    data: &State<AppState>,
) -> Result<Json<SingleEventRiderResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::eventrider::dsl::*;
    let connection = &mut establish_connection();
    let event_id_clone = event_id.clone();
    let rider_id_clone = rider_id.clone();


    let result = eventrider
        .find((event_id_clone,rider_id_clone))
        .first::<EventRider>(connection)
        .ok();

    match result {
        Some(eventrider_x) => {
            let json_response = SingleEventRiderResponse {
                status: "success".to_string(),
                eventrider: EventRiderData {
                    eventrider: eventrider_x.to_owned(),
                },
            };
            return Ok(Json(json_response));
        }
        None => {
            let error = GenericResponse {
                status: "error".to_string(),
                message: "EventRider not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(error)));
        }
    }

}

#[patch("/eventrider?<event_id>&<rider_id>", data = "<body>")]
pub async fn update_eventrider_handler(
    event_id: String,
    rider_id: String,
    body: Json<UpdateEventRider>,
    data: &State<AppState>,
) -> Result<Json<SingleEventRiderResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::eventrider::dsl::*;
    let connection = &mut establish_connection();
    let event_id_clone = event_id.clone();
    let rider_id_clone = rider_id.clone();
    let result = eventrider
        .find((event_id_clone,rider_id_clone))
        .first::<EventRider>(connection)
        .ok();
    
    match result {
        None => {
            let error = GenericResponse {
                status: "error".to_string(),
                message: "EventRider not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(error)));
        }
        Some(old_eventrider) => {
            let datetime = Utc::now().naive_utc();
            
            let er_er_type = body.er_type.to_owned().unwrap_or(old_eventrider.er_type.to_owned());
            let er_er_specialization = body.er_specialization.to_owned().unwrap_or(old_eventrider.er_specialization.to_owned());
            
            let payload = EventRider {
            e_id: old_eventrider.e_id.to_owned(),
            r_id: old_eventrider.r_id.to_owned(),
            er_type: if !er_er_type.is_empty() { 
                er_er_type 
            } else {
                old_eventrider.er_type.to_owned()
            },
            er_specialization: if !er_er_specialization.is_empty() { 
                er_er_specialization 
            } else {
                old_eventrider.er_specialization.to_owned()
            },created_at: old_eventrider.created_at.to_owned(),
            updated_at: Some(datetime),
            };

            let connection = &mut establish_connection();
            diesel::update(eventrider.find((old_eventrider.e_id.clone(),old_eventrider.r_id.clone())))
            .set(&payload)
            .execute(connection)
            .expect("Error updating eventrider");
            let json_response = SingleEventRiderResponse {
                status: "success".to_string(),
                eventrider: EventRiderData {
                    eventrider: payload.clone(),
                }
            };
        return Ok(Json(json_response));
        }
        
        
    }
}

pub fn delete_eventrider_dependencies (
    event_id: String,
    rider_id: String)-> Result<usize, diesel::result::Error>{
        Ok(1)
    }

#[delete("/eventrider?<event_id>&<rider_id>")]
pub async fn delete_eventrider_handler(
    event_id: String,
    rider_id: String,
    data: &State<AppState>,
) -> Result<Json<GenericResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::eventrider::dsl::*;
    let connection = &mut establish_connection();
    let event_id_clone = event_id.clone();
    let rider_id_clone = rider_id.clone();

    match diesel::delete(eventrider.find((event_id_clone,rider_id_clone)))
        .execute(connection){
        Ok(_) => {
            let response_json = GenericResponse {
                status: "success".to_string(),
                message: "EventRider deleted".to_string(),
            };

            return Ok(Json(response_json));
        }
        Err(_) => {
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "EventRider not found".to_string(),
            };

            return Err(Custom(Status::NotFound, Json(response_json)));
        }
    }
}