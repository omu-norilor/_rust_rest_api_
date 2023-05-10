use crate::{
    model::{Rider,RiderStat,UpdateRider,Bike,Helmet,EventRider,AppState},
    response::{GenericResponse, RiderData, SingleRiderWithGearResponse,RiderListResponse,RiderStatListResponse},
    // handlers::eventriderhandler::delete_eventrider_dependencies,
    db::establish_connection,
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


#[openapi(tag = "Riders")]
#[get("/riders/getcount")]
pub async fn riders_count_handler(data: &State<AppState>) -> Result<Json<GenericResponse>, Status> {
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();

    //get the count of riders
    let count = riders
        .count()
        .get_result::<i64>(connection)
        .expect("Error loading riders");

    //send the count back
    let response_json = GenericResponse {
        status: "success".to_string(),
        message: count.to_string(),
    };

    Ok(Json(response_json))
}


#[openapi(tag = "Riders")]
#[get("/riders/getall?<page>&<limit>")]
pub async fn riders_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<RiderStatListResponse>, Status> {
    
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let limit = limit.map(|l| l as i64).unwrap_or(10);
    let page = page.map(|p| p as i64).unwrap_or(1);
    let offset = (page - 1) * limit;

    let vec = riders
        .load::<Rider>(connection)
        .expect("Error loading riders");
    
    let vec = riders
        .limit(limit)
        .offset(offset)
        .load::<Rider>(connection)
        .expect("Error loading helmets");

    let count = riders
        .count()
        .get_result::<i64>(connection)
        .expect("Error loading helmets");


    //get the count of riders for each bike
    let mut rider_stat_vec: Vec<RiderStat> = Vec::new();
    for rider in vec.clone(){
        let no_events = get_no_events_for_rider(rider.r_id.clone());
        let rider_stat = RiderStat {
            r_id: rider.r_id.clone(),
            r_name: rider.r_name.clone(),
            email: rider.email.clone(),
            phone: rider.phone.clone(),
            specialization: rider.specialization.clone(),
            bike_id: rider.bike_id.clone(),
            helmet_id: rider.helmet_id.clone(),
            height: rider.height.clone(),
            r_weight: rider.r_weight.clone(),
            created_at: rider.created_at.clone(),
            updated_at: rider.updated_at.clone(),
            no_events: no_events,
        };
        rider_stat_vec.push(rider_stat);
    }

    let json_response = RiderStatListResponse {
        status: "success".to_string(),
        results: count,
        riders: rider_stat_vec,
    };

    return Ok(Json(json_response));
}

pub fn specialization_validation( r_specialization: String) -> bool{
    let mut valid = false;
    if r_specialization == "Freeride" || r_specialization == "Road" || r_specialization == "Trail" || r_specialization == "Enduro" || r_specialization == "Downhill" || r_specialization == "Cross-country" || r_specialization == "Dirt" || r_specialization == "BMX" || r_specialization == "Other"{
        valid = true;
    }
    valid
}
use regex::Regex;
pub fn email_validation( r_email: String) -> bool{
    let re = regex::Regex::new(r"\S+@\S+\.\S+").unwrap();
    re.is_match(r_email.as_str())
}

pub fn phone_validation( r_phone: String) -> bool{
    let mut valid = true;

    // if phone doesn't have 10 digits then false
    if r_phone.len() != 10{
        valid = false;
    }
    // if phone has non numeric characters then false
    if r_phone.chars().any(|c| !c.is_numeric()) {
        valid = false;
    }
    valid
}

#[openapi(tag = "Riders")]
#[post("/riders/new", data = "<body>")]
pub async fn create_rider_handler(
    mut body: Json<Rider>,
    data: &State<AppState>,
) -> Result<Json<SingleRiderWithGearResponse>, Custom<Json<GenericResponse>>>{
    
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let vec = riders
        .load::<Rider>(connection)
        .expect("Error loading riders");

    // check validations
    if !specialization_validation(body.specialization.clone()) || !phone_validation(body.phone.clone()){
        let response_json = GenericResponse {
            status: "error".to_string(),
            message: "Specialization or phone is not valid".to_string(),
        };
        return Err(Custom(Status::BadRequest, Json(response_json)));
    }

    // check if rider already exists
    for rider in vec.iter() {
        if rider.r_name == body.r_name && rider.email==rider.email && rider.phone == body.phone{
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Rider already exists DING DONG".to_string(),
            };
            return Err(Custom(Status::BadRequest, Json(response_json)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now().naive_utc();
    body.created_at = datetime;
    body.updated_at = Some(datetime);
    let new_rider =Rider{
        r_id: uuid_id.to_string(),
        helmet_id: body.helmet_id.clone(),
        bike_id: body.bike_id.clone(),
        r_name: body.r_name.clone(),
        height: body.height,
        r_weight: body.r_weight,
        specialization: body.specialization.clone(),
        email: body.email.clone(),
        phone: body.phone.clone(),
        created_at:body.created_at.clone(),
        updated_at: Some(datetime.clone()),
    };


    let rider = new_rider.to_owned(); 
    let rider_for_db = rider.clone();

    let connection = &mut establish_connection();
    diesel::insert_into(riders)
    .values(&rider_for_db)
    .execute(connection)
    .expect("Error saving new rider PAOAO");
    let h_id = rider.helmet_id.clone();
    let b_id = rider.bike_id.clone();
    let res_bike = get_bike_for_rider(b_id).unwrap();
    let res_helmet = get_helmet_for_rider(h_id).unwrap();
    let json_response =SingleRiderWithGearResponse{
        status: "success".to_string(),
        rider: RiderData {
            rider: rider,
        },
        bike: res_bike,
        helmet: res_helmet,
    };

    Ok(Json(json_response))
}

pub fn get_helmet_for_rider(helmetid: String) -> Option<Helmet> {
    use crate::schema::helmets::dsl::*;
    let connection = &mut establish_connection();
    let helmet_id_clone = helmetid.clone();
    let result = helmets
        .find(helmet_id_clone)
        .first::<Helmet>(connection)
        .ok();
    result.clone()
}
pub fn get_bike_for_rider(bikeid: String) -> Option<Bike> {
    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let bike_id_clone = bikeid.clone();
    let result = bikes
        .find(bike_id_clone)
        .first::<Bike>(connection)
        .ok();
    result.clone()
}

#[openapi(tag = "Riders")]
#[get("/riders/get/<rider_id>")]
pub async fn get_rider_handler(
    rider_id: String,
    data: &State<AppState>,
) -> Result<Json<SingleRiderWithGearResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let rider_id_clone = rider_id.clone();
    let result = riders
        .find(rider_id_clone)
        .first::<Rider>(connection)
        .ok();

    match result {
        Some(rider) => {
            let h_id = rider.helmet_id.clone();
            let b_id = rider.bike_id.clone();
            let res_bike = get_bike_for_rider(b_id).unwrap();
            let res_helmet = get_helmet_for_rider(h_id).unwrap();
            
            let json_response = SingleRiderWithGearResponse {
                status: "success".to_string(),
                rider: RiderData {
                    rider: rider.to_owned(),
                },
                bike: res_bike,
                helmet: res_helmet,
            };
            return Ok(Json(json_response));
        }
        None => {
            let error = GenericResponse {
                status: "error".to_string(),
                message: "Rider not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(error)));
        }
    }

}

#[openapi(tag = "Riders")]
#[post("/riders/edit/<rider_id>", data = "<body>")]
pub async fn update_rider_handler(
    rider_id: String,
    body: Json<UpdateRider>,
    data: &State<AppState>,
) -> Result<Json<SingleRiderWithGearResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let rider_id_clone = rider_id.clone();
    let result = riders
        .find(rider_id_clone)
        .first::<Rider>(connection)
        .ok();
    
    match result {
        None => {
            let error = GenericResponse {
                status: "error".to_string(),
                message: "Rider not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(error)));
        }
        Some(old_rider) => {
            let datetime = Utc::now().naive_utc();
            let r_r_name = body.r_name.to_owned().unwrap_or(old_rider.r_name.to_owned());
            let r_email = body.email.to_owned().unwrap_or(old_rider.email.to_owned());
            let r_phone = body.phone.to_owned().unwrap_or(old_rider.phone.to_owned());
            let r_specialization = body.specialization.to_owned().unwrap_or(old_rider.specialization.to_owned());
            let r_bike_id = body.bike_id.to_owned().unwrap_or(old_rider.bike_id.to_owned());
            let r_helmet_id = body.helmet_id.to_owned().unwrap_or(old_rider.helmet_id.to_owned());
            
            let payload = Rider {
            r_id: old_rider.r_id.to_owned(),
            r_name: if !r_r_name.is_empty() { 
                r_r_name 
            } else {
                old_rider.r_name.to_owned()
            },
            email: if !r_email.is_empty() { 
                r_email 
            } else {
                old_rider.email.to_owned()
            },
            phone: if !r_phone.is_empty() { 
                r_phone
            } else {
                old_rider.phone.to_owned()
            },
            specialization: if !r_specialization.is_empty() { 
                r_specialization 
            } else {
                old_rider.specialization.to_owned()
            },
            bike_id: if r_bike_id.is_empty() { 
                r_bike_id
            } else {
                old_rider.bike_id.to_owned()
            },
            helmet_id: if r_helmet_id.is_empty() { 
                r_helmet_id
            } else {
                old_rider.helmet_id.to_owned()
            },
            r_weight: body.r_weight.to_owned().unwrap_or(old_rider.r_weight),
            height: body.height.to_owned().unwrap_or(old_rider.height),
            created_at: old_rider.created_at.to_owned(),
            updated_at: Some(datetime),
            };

            //check validations
            if !specialization_validation(payload.specialization.to_owned()) || !phone_validation(payload.phone.to_owned()) {
                let error = GenericResponse {
                    status: "error".to_string(),
                    message: "Specialization not valid".to_string(),
                };
                return Err(Custom(Status::BadRequest, Json(error)));
            }

            let connection = &mut establish_connection();
            diesel::update(riders.find(old_rider.r_id.clone()))
            .set(&payload)
            .execute(connection)
            .expect("Error updating rider");
            
            let h_id = payload.helmet_id.clone();
            let b_id = payload.bike_id.clone();
            let res_bike = get_bike_for_rider(b_id).unwrap();
            let res_helmet = get_helmet_for_rider(h_id).unwrap();

            let json_response = SingleRiderWithGearResponse {
                status: "success".to_string(),
                rider: RiderData {
                    rider: payload.clone(),
                },
                bike: res_bike,
                helmet: res_helmet,
        };
        return Ok(Json(json_response));
        }
        
        
    }
}

// pub fn delete_rider_dependencies(rider_id: String) -> Result<usize, diesel::result::Error>{
//     use crate::schema::eventrider::dsl::*;
//     let connection = &mut establish_connection();
//     let result = eventrider
//         .filter(r_id.eq(rider_id.clone()))
//         .load::<EventRider>(connection)
//         .expect( "Error loading riders");

//     for event_rider in result{
//        match  delete_eventrider_dependencies(event_rider.e_id.clone(), event_rider.r_id.clone()){
//            Ok(_) => {
//            }
//            Err(_) => {
//                 return Err(diesel::result::Error::NotFound);
//            }
//         }
//     }

//     let connection = &mut establish_connection();
//     let rider_id_clone = rider_id.clone();
//     diesel::delete(eventrider.filter(r_id.eq(rider_id_clone))).execute(connection)
// }

#[openapi(tag = "Riders")]
#[post("/riders/delete/<rider_id>")]
pub async fn delete_rider_handler(
    rider_id: String,
    data: &State<AppState>,
) -> Result<Json<GenericResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let rider_id_clone = rider_id.clone();

     match diesel::delete(riders.find(rider_id_clone))
        .execute(connection){
        Ok(_) => {
            let response_json = GenericResponse {
                status: "success".to_string(),
                message: "Rider deleted".to_string(),
            };

            return Ok(Json(response_json));
        }
        Err(_) => {
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Rider not found".to_string(),
            };

            return Err(Custom(Status::NotFound, Json(response_json)));
        }
    }

    // match delete_rider_dependencies(rider_id.clone()){
    //     Ok(_) => {
    //         match diesel::delete(riders.find(rider_id_clone)).execute(connection){
    //             Ok(_) => {
    //                 let response_json = GenericResponse {
    //                 status: "success".to_string(),
    //                 message: "Rider deleted".to_string(),
    //                 };
    //                 return Ok(Json(response_json));
    //             }
    //             Err(_) => {
    //                 let response_json = GenericResponse {
    //                     status: "error".to_string(),
    //                     message: "Rider not found".to_string(),
    //                 };
    //                 return Err(Custom(Status::NotFound, Json(response_json)));
    //             }
    //         }
    //     },
    //     Err(_) => {
    //         let response_json = GenericResponse {
    //             status: "error".to_string(),
    //             message: "Error at rider dependency deletion".to_string(),
    //         };
    //         return Err(Custom(Status::NotFound, Json(response_json)));
    //     }
    // }

}



pub fn get_no_events_for_rider(riderid: String) -> i64 {
    
    use crate::schema::eventrider::dsl::*;
    let connection = &mut establish_connection();
    let rider_id_clone = riderid.clone();
    let result = eventrider
        .filter(r_id.eq(rider_id_clone))
        .count()
        .get_result::<i64>(connection)
        .expect("Error loading eventriders");
    result.clone()
}

#[openapi(tag = "Riders")]
#[get("/riders/mostactive?<page>&<limit>")]
pub async fn get_most_active_riders_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<RiderStatListResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::{riders,eventrider};
    use diesel::dsl::{count, max};
    use diesel::prelude::*;

    let connection = &mut establish_connection();
    let limit = limit.map(|l| l as i64).unwrap_or(10);
    let page = page.map(|p| p as i64).unwrap_or(1);
    let offset = (page - 1) * limit;

    let vec =riders::table
        .left_join(
            eventrider::table.on(riders::r_id.eq(eventrider::r_id)),
        )
        .select(riders::all_columns)
        .group_by(riders::r_id)
        .order_by(count(eventrider::r_id).desc())
        .limit(limit)
        .offset(offset)
        .load::<Rider>(connection)
        .expect("Error loading event-rider counts");

    let count = riders::table
    .count()
    .get_result::<i64>(connection)
    .expect("Error loading rider count");
    let mut rider_stat_vec: Vec<RiderStat> = Vec::new();
    for rider in vec.clone(){
        let no_events = get_no_events_for_rider(rider.r_id.clone());
        let rider_stat = RiderStat {
            r_id: rider.r_id.clone(),
            r_name: rider.r_name.clone(),
            email: rider.email.clone(),
            phone: rider.phone.clone(),
            specialization: rider.specialization.clone(),
            bike_id: rider.bike_id.clone(),
            helmet_id: rider.helmet_id.clone(),
            height: rider.height.clone(),
            r_weight: rider.r_weight.clone(),
            created_at: rider.created_at.clone(),
            updated_at: rider.updated_at.clone(),
            no_events: no_events,
        };
        rider_stat_vec.push(rider_stat);
    }

    let json_response = RiderStatListResponse {
        status: "success".to_string(),
        results: count,
        riders: rider_stat_vec,
    };

    return Ok(Json(json_response));
}