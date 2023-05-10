use crate::{
    model::{Bike,BikeStat, UpdateBike,Rider,AppState},
    response::{BikeData, BikeListResponse, GenericResponse, SingleBikeResponse, SingleBikeWRidersResponse,BikeStatListResponse},
    // handlers::riderhandler::{delete_rider_dependencies},
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
#[get("/bikes/getcount")]
pub async fn bikes_count_handler(data: &State<AppState>) -> Result<Json<GenericResponse>, Status> {
    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();

    let count = bikes
        .count()
        .get_result::<i64>(connection)
        .expect("Error loading bikes count");
    let json_response = GenericResponse {
        status: "success".to_string(),
        message: count.to_string(),
    };
    Ok(Json(json_response))
}

pub fn get_no_riders_for_bike(bikeid: String) -> i64 {
    
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let bike_id_clone = bikeid.clone();
    let result = riders
        .filter(bike_id.eq(bike_id_clone))
        .count()
        .get_result::<i64>(connection)
        .expect("Error loading riders \n");
    print!("Rider count for bike {} is {}", bikeid, result);
    result.clone()
}

#[openapi(tag = "Bikes")]
#[get("/bikes/getall?<page>&<limit>")]
pub async fn bikes_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<BikeStatListResponse>, Status> {
   
    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let limit = limit.map(|l| l as i64).unwrap_or(10);
    let page = page.map(|p| p as i64).unwrap_or(1);
    let offset = (page - 1) * limit;


    let vec = bikes
        .limit(limit)
        .offset(offset)
        .load::<Bike>(connection)
        .expect("Error loading bikes");

    let count = bikes
        .count()
        .get_result::<i64>(connection)
        .expect("Error loading bikes count");
    //get the count of bikes
    //send the count back
    let mut bike_stat_vec: Vec<BikeStat> = Vec::new();
    for bike in vec.clone(){
        let no_riders = get_no_riders_for_bike(bike.b_id.clone());
        let bike_stat = BikeStat {
            b_id: bike.b_id.clone(),
            brand: bike.brand.clone(),
            model: bike.model.clone(),
            size: bike.size.clone(),
            wheelsize: bike.wheelsize.clone(),
            sold: bike.sold.clone(),
            created_at: bike.created_at.clone(),
            updated_at: bike.updated_at.clone(),    
            price: bike.price.clone(),
            
            no_riders: no_riders,
        };
        bike_stat_vec.push(bike_stat);
    }
    let json_response = BikeStatListResponse {
        status: "success".to_string(),
        results: count,
        bikes: bike_stat_vec,
    };
    Ok(Json(json_response))
}

#[openapi(tag = "Bikes")]
#[get("/bikes/filter?<comp>&<page>&<limit>&<bike_price>")]
pub async fn bikes_filter_handler(
    comp: String,
    bike_price: f64,
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<BikeStatListResponse>, Custom<Json<GenericResponse>>> {

    
    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let limit = limit.map(|l| l as i64).unwrap_or(10);
    let page = page.map(|p| p as i64).unwrap_or(1);
    let offset = (page - 1) * limit;
    let vec:Vec<Bike>;
    let count:i64;
    
    match comp.as_str() {
        "gt" => {
            vec = bikes.
            filter(price.gt(bike_price))
            .limit(limit)
            .offset(offset)
            .load::<Bike>(connection)
            .expect("Error loading bikes");

            count = bikes
            .filter(price.gt(bike_price))
            .count()
            .get_result::<i64>(connection)
            .expect("Error loading bikes count");
                },
        "lt" => {
            vec = bikes.
            filter(price.lt(bike_price))
            .limit(limit)
            .offset(offset)
            .load::<Bike>(connection)
            .expect("Error loading bikes");

            count = bikes
            .filter(price.lt(bike_price))
            .count()
            .get_result::<i64>(connection)
            .expect("Error loading bikes count");
        },
        "eq" => {
            vec = bikes.
            filter(price.eq(bike_price))
            .limit(limit)
            .offset(offset)
            .load::<Bike>(connection)
            .expect("Error loading bikes");

            count = bikes
            .filter(price.eq(bike_price))
            .count()
            .get_result::<i64>(connection)
            .expect("Error loading bikes count");
        },
        _ => {
            let erorr = GenericResponse {
                status: "error".to_string(),
                message: "No comparison".to_string(),
            };
            return Err(Custom(Status::BadRequest, Json(erorr)));
        }
    }
    let mut bike_stat_vec: Vec<BikeStat> = Vec::new();
    for bike in vec.clone(){
        let no_riders = get_no_riders_for_bike(bike.b_id.clone());
        let bike_stat = BikeStat {
            b_id: bike.b_id.clone(),
            brand: bike.brand.clone(),
            model: bike.model.clone(),
            size: bike.size.clone(),
            wheelsize: bike.wheelsize.clone(),
            sold: bike.sold.clone(),
            created_at: bike.created_at.clone(),
            updated_at: bike.updated_at.clone(),    
            price: bike.price.clone(),
            
            no_riders: no_riders,
        };
        bike_stat_vec.push(bike_stat);
    }
    let json_response: BikeStatListResponse = BikeStatListResponse {
        status: "success".to_string(),
        results: count,
        bikes: bike_stat_vec,
    };
    Ok(Json(json_response))
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
#[post("/bikes/new", data = "<body>")]
pub async fn create_bike_handler(
    mut body: Json<Bike>,
    data: &State<AppState>,
) -> Result<Json<SingleBikeResponse>, Custom<Json<GenericResponse>>>{
    
    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    
    // let vec = bikes
    //     .load::<Bike>(connection)
    //     .expect("Error loading bikes");
    // for bike in vec.iter() {
    //     if bike.brand == body.brand && body.model==bike.model && bike.sold.eq(&false){
    //         let error = GenericResponse {
    //             status: "error".to_string(),
    //             message: "Bike already exists".to_string(),
    //         };
    //         return Err(Custom(Status::Conflict, Json(error)));
    //     }
    // }

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
#[get("/bikes/get/<bike_id>")]
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
#[post("/bikes/edit/<bike_id>", data = "<body>")]
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
                .expect("Error updating bike");
                
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

// pub fn delete_bike_dependencies(bikeid: String,data: &State<AppState>) -> Result<usize, diesel::result::Error> {
//     use crate::schema::riders::dsl::*;
//     let connection = &mut establish_connection();
//     let result =riders 
//         .filter(bike_id.eq(bike_id.clone()))
//         .load::<Rider>(connection).expect( "Error loading riders");

//     // delete all riders that have the bike id that is being deleted
//     for rider in result.clone(){
//         match delete_rider_dependencies(rider.r_id.clone()){
//             Ok(_) =>{

//             }
//             Err(e) => {
//                 return Err(e);
//             }
//         }
//     }
//     let bike_id_clone = bikeid.clone();
//     let connection = &mut establish_connection();
//     diesel::delete(riders.filter(bike_id.eq(bike_id_clone))).execute(connection)


// }

#[openapi(tag = "Bikes")]
#[post("/bikes/delete/<bike_id>")]
pub async fn delete_bike_handler(
    bike_id: String,
    data: &State<AppState>,
) -> Result<Json<GenericResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::bikes::dsl::*;
    let connection = &mut establish_connection();
    let bike_id_clone = bike_id.clone();
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

    // match delete_bike_dependencies(bike_id.clone(),data){
    //     Ok(_) => {
    //         match diesel::delete(bikes.find(bike_id_clone)).execute(connection){
    //             Ok(_) => {
    //                 let response_json = GenericResponse {
    //                 status: "success".to_string(),
    //                 message: "Bike deleted".to_string(),
    //                 };
    //                 return Ok(Json(response_json));
    //             }
    //             Err(_) => {
    //                 let response_json = GenericResponse {
    //                     status: "error".to_string(),
    //                     message: "Bike not found".to_string(),
    //                 };
    //                 return Err(Custom(Status::NotFound, Json(response_json)));
    //             }
    //         }
    //     },
    //     Err(_) => {
    //         let response_json = GenericResponse {
    //             status: "error".to_string(),
    //             message: "Error at bike dependency deletion".to_string(),
    //         };
    //         return Err(Custom(Status::NotFound, Json(response_json)));
    //     }
    // }


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
    
        let mut rider_counts = Vec::new();
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
            rider_counts.push(0);
        }
    }
    

    let response_json = BikeListResponse {
        status: "success".to_string(),
        results: passed.len(),
        bikes:passed,
        counts: rider_counts,
    };

    Ok(Json(response_json))
}