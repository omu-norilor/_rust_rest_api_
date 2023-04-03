use crate::{
    model::{Bike, UpdateBike,Rider,AppState},
    response::{BikeData, BikeListResponse, GenericResponse, SingleBikeResponse, SingleBikeWRidersResponse},
    handlers::riderhandler::{delete_rider_dependencies},
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

#[openapi(tag = "Bikes")]
#[get("/bikes?<page>&<limit>")]
pub async fn bikes_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<BikeListResponse>, Status> {
   
    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let vec = bikes
        .filter(sold.eq(false))
        .load::<Bike>(connection)
        .expect("Error loading bikes");
    
    let mut limit = limit.unwrap_or(10);
    let mut offset = (page.unwrap_or(1) - 1) * limit;
    let good_bikes: Vec<Bike> = vec.clone().into_iter().skip(offset).take(limit).collect();
    let response_json = BikeListResponse {
        status: "success".to_string(),
        results: good_bikes.len(),
        bikes:good_bikes
    };

    Ok(Json(response_json))
}

#[openapi(tag = "Bikes")]
#[options("/bikes/<comp>/<bike_price>")]
pub async fn bikes_filter_handler(
    comp: String,
    bike_price: f64,
    data: &State<AppState>,
) -> Result<Json<BikeListResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let vec = bikes
        .load::<Bike>(connection)
        .expect("Error loading bikes");

    let vec_clone:Vec<Bike>;
    match comp.as_str() {
        "gt" => {
            vec_clone = vec.clone().iter().filter(|bike| bike.sold.eq(&false) && bike.price.gt(&bike_price)).map(|bike|bike.clone()).collect();
        },
        "lt" => {
            vec_clone = vec.clone().iter().filter(|bike| bike.sold.eq(&false) && bike.price.lt(&bike_price)).map(|bike|bike.clone()).collect();

        },
        "eq" => {
            vec_clone = vec.clone().iter().filter(|bike| bike.sold.eq(&false) && bike.price.eq(&bike_price)).map(|bike|bike.clone()).collect();

        },
        _ => {
            let erorr = GenericResponse {
                status: "error".to_string(),
                message: "No comparison".to_string(),
            };
            return Err(Custom(Status::BadRequest, Json(erorr)));
        }
    }
    for bike in vec_clone.iter() {
    
    }
    let response_json = BikeListResponse {
        status: "success".to_string(),
        results: vec_clone.len(),
        bikes:vec_clone.clone()
    };

    Ok(Json(response_json))
}

pub fn size_validation(b_size: String) -> bool {
    let mut valid = false;
    if b_size == "XS" || b_size == "S" || b_size == "M" || b_size == "L" || b_size == "XL" || b_size == "XXL"{
        valid = true;
    }
    valid
}
pub fn wheelsize_validation(b_wheelsize: f64) -> bool {
    let mut valid = false;
    if b_wheelsize == 26.0 || b_wheelsize == 27.5 || b_wheelsize == 29.0 || b_wheelsize == 20.0 || b_wheelsize == 24.0 || b_wheelsize == 28.0{
        valid = true;
    }
    valid
}

#[openapi(tag = "Bikes")]
#[post("/bikes", data = "<body>")]
pub async fn create_bike_handler(
    mut body: Json<Bike>,
    data: &State<AppState>,
) -> Result<Json<SingleBikeResponse>, Custom<Json<GenericResponse>>>{
    
    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    
    let vec = bikes
        .load::<Bike>(connection)
        .expect("Error loading bikes");

    for bike in vec.iter() {
        if bike.brand == body.brand && body.model==bike.model && bike.sold.eq(&false){
            let error = GenericResponse {
                status: "error".to_string(),
                message: "Bike already exists".to_string(),
            };
            return Err(Custom(Status::Conflict, Json(error)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now().naive_utc();
    body.created_at = datetime.clone();
    body.updated_at = Some(datetime.clone());
    body.sold = false;
    let new_bike = Bike{
        b_id: uuid_id.to_string(),
        brand: body.brand.clone(),
        model: body.model.clone(),
        wheelsize: body.wheelsize.clone(),
        size: body.size.clone(),
        price: body.price,
        created_at: body.created_at.clone(),
        updated_at: Some(datetime.clone()),
        sold: body.sold
    };
    //check validations
    if !size_validation(new_bike.size.clone()) || !wheelsize_validation(new_bike.wheelsize.clone()){
        let error = GenericResponse {
            status: "error".to_string(),
            message: "Invalid size".to_string(),
        };
        return Err(Custom(Status::BadRequest, Json(error)));
    }
    let bike = new_bike.to_owned();
    let bike_for_db=bike.clone();
    let connection = &mut establish_connection();
    diesel::insert_into(bikes)
        .values(&bike_for_db)
        .execute(connection)
        .expect("Error saving new bike");

    let json_response =SingleBikeResponse{
        status: "success".to_string(),
        bike: BikeData {
            bike: bike,
        },
    };

    Ok(Json(json_response))
}

pub fn get_riders_for_bike(bikeid: String) -> Vec<Rider> {
    
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let bike_id_clone = bikeid.clone();
    let result = riders
        .filter(bike_id.eq(bike_id_clone))
        .load::<Rider>(connection)
        .expect("Error loading riders");
    result.clone()
}

#[openapi(tag = "Bikes")]
#[get("/bikes/<bike_id>")]
pub async fn get_bike_handler(
    bike_id: String,
    data: &State<AppState>,
) -> Result<Json<SingleBikeWRidersResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let bike_id_clone = bike_id.clone();
    let result = bikes
        .find(bike_id_clone)
        .first::<Bike>(connection)
        .ok();

    match result{
        Some(bike) => {
            let riders = get_riders_for_bike(bike.b_id.clone());
            let json_response = SingleBikeWRidersResponse {
                status: "success".to_string(),
                bike: BikeData {
                    bike: bike.to_owned(),
                },
                riders: riders
            };
            return Ok(Json(json_response));
        },
        None => {
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Bike not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(response_json)));
        }
    }
}


#[openapi(tag = "Bikes")]
#[patch("/bikes/<bike_id>", data = "<body>")]
pub async fn update_bike_handler(
    bike_id: String,
    body: Json<UpdateBike>,
    data: &State<AppState>,
) -> Result<Json<SingleBikeResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let bike_id_clone = bike_id.clone();
    let res = bikes
        .find(bike_id_clone)
        .first::<Bike>(connection)
        .ok();
    match res{
        None => {
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Bike not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(response_json)));
        }
        Some(old_bike) => {
            let datetime = Utc::now().naive_utc();
            let datetime = Utc::now().naive_utc();
            let b_brand = body.brand.to_owned().unwrap_or(old_bike.brand.to_owned());
            let b_model = body.model.to_owned().unwrap_or(old_bike.model.to_owned());
            let b_size = body.size.to_owned().unwrap_or(old_bike.size.to_owned());

            let payload = Bike {
                b_id: old_bike.b_id.to_owned(),
                brand: if !b_brand.is_empty() { 
                    b_brand 
                } else {
                    old_bike.brand.to_owned()
                },
                model: if !b_model.is_empty() { 
                    b_model 
                } else {
                    old_bike.model.to_owned()
                },
                wheelsize: if body.wheelsize.is_some() { 
                    body.wheelsize.unwrap()
                } else {
                    old_bike.wheelsize
                },
                size: if !b_size.is_empty() { 
                    b_size 
                } else {
                    old_bike.size.to_owned()
                },
                price: if body.price.is_some() { 
                    body.price.unwrap()
                } else {
                    old_bike.price
                },
                sold: if body.sold.is_some() { 
                    body.sold.unwrap()
                } else {
                    old_bike.sold
                },
                created_at: old_bike.created_at.to_owned(),
                updated_at: Some(datetime),
                };

                if !size_validation(payload.size.clone()) || !wheelsize_validation(payload.wheelsize.clone()){
                    let error = GenericResponse {
                        status: "error".to_string(),
                        message: "Invalid size".to_string(),
                    };
                    return Err(Custom(Status::BadRequest, Json(error)));
                }
                let connection = &mut establish_connection();
                diesel::update(bikes.find(old_bike.b_id.clone()))
                .set(&payload)
                .execute(connection)
                .expect("Error updating helmet");
                
                let json_response = SingleBikeResponse {
                    status: "success".to_string(),
                    bike: BikeData {
                        bike: payload.clone(),
                    },
                };
                return Ok(Json(json_response));

        }   
    }
}

pub fn delete_bike_dependencies(bikeid: String,data: &State<AppState>) -> Result<usize, diesel::result::Error> {
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let result =riders 
        .filter(bike_id.eq(bike_id.clone()))
        .load::<Rider>(connection).expect( "Error loading riders");

    // delete all riders that have the bike id that is being deleted
    for rider in result.clone(){
        match delete_rider_dependencies(rider.r_id.clone()){
            Ok(_) =>{

            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    let bike_id_clone = bikeid.clone();
    let connection = &mut establish_connection();
    diesel::delete(riders.filter(bike_id.eq(bike_id_clone))).execute(connection)


}

#[openapi(tag = "Bikes")]
#[delete("/bikes/<bike_id>")]
pub async fn delete_bike_handler(
    bike_id: String,
    data: &State<AppState>,
) -> Result<Json<GenericResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let bike_id_clone = bike_id.clone();

    match delete_bike_dependencies(bike_id.clone(),data){
        Ok(_) => {
            match diesel::delete(bikes.find(bike_id_clone)).execute(connection){
                Ok(_) => {
                    let response_json = GenericResponse {
                    status: "success".to_string(),
                    message: "Bike deleted".to_string(),
                    };
                    return Ok(Json(response_json));
                }
                Err(_) => {
                    let response_json = GenericResponse {
                        status: "error".to_string(),
                        message: "Bike not found".to_string(),
                    };
                    return Err(Custom(Status::NotFound, Json(response_json)));
                }
            }
        },
        Err(_) => {
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Error at bike dependency deletion".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(response_json)));
        }
    }


}


#[openapi(tag = "Bikes")]
#[post("/moreBikes" , data = "<body>")]
pub async fn create_more_bikes_handler(
    body: Json<Vec<Bike>>,
    data: &State<AppState>,
) -> Result<Json<BikeListResponse>, Status>{

    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let mut passed : Vec<Bike> = Vec::new();
    let vec = bikes
        .load::<Bike>(connection)
        .expect("Error loading bikes");

    for json_bike in body.into_inner().clone(){
        let mut found = false;
        for db_bike in vec.clone(){
            if json_bike.brand == db_bike.brand && json_bike.model==db_bike.model{
                found = true;
            }
        }
        if !found{
            let datetime = Utc::now().naive_utc();
            let u = Uuid::new_v4().to_string();
            let payload = Bike {
                b_id: u.clone(),
                brand: json_bike.brand.to_owned(),
                model: json_bike.model.to_owned(),
                wheelsize: json_bike.wheelsize,
                size: json_bike.size.to_owned(),
                price: json_bike.price,
                sold: json_bike.sold,
                created_at: datetime,
                updated_at: None,
            };
            if size_validation(payload.size.clone()) && wheelsize_validation(payload.wheelsize.clone()){
                let connection = &mut establish_connection();
                diesel::insert_into(bikes)
                    .values(&payload)
                    .execute(connection)
                    .expect("Error inserting bike");

            }
            passed.push(payload.clone());
        }
    }
    

    let response_json = BikeListResponse {
        status: "success".to_string(),
        results: passed.len(),
        bikes:passed
    };

    Ok(Json(response_json))
}