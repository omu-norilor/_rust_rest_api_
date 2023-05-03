use crate::{
    model::{Helmet, UpdateHelmet,Rider, AppState},
    response::{HelmetData, HelmetListResponse, GenericResponse, SingleHelmetResponse,HelmetStatListResponse,HelmetStat},
    handlers::riderhandler::{delete_rider_dependencies},
    db::establish_connection,
};

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use diesel::prelude::*;
use chrono::prelude::*;
use uuid::Uuid;

use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State
};

#[openapi(tag = "Helmets")]
#[get("/helmets/getcount")]
pub async fn helmets_count_handler(data: &State<AppState>) -> Result<Json<GenericResponse>, Status> {
    use crate::schema::helmets::dsl::*;
    let connection = &mut establish_connection();

    //get the count of helmets
    let count = helmets
        .count()
        .get_result::<i64>(connection)
        .expect("Error loading helmets");

    //send the count back
    let response_json = GenericResponse {
        status: "success".to_string(),
        message: count.to_string(),
    };

    Ok(Json(response_json))
}


#[openapi(tag = "Helmets")]
#[get("/helmets/getall?<page>&<limit>")]
pub async fn helmets_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<HelmetListResponse>, Status> {
    
    use crate::schema::helmets::dsl::*;
    let connection = &mut establish_connection();
    let vec = helmets
        .load::<Helmet>(connection)
        .expect("Error loading helmets");
    
    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;
    let good_helmets: Vec<Helmet> = vec.clone().into_iter().skip(offset).take(limit).collect();

    let response_json = HelmetListResponse {
        status: "success".to_string(),
        results: vec.len(),
        helmets:good_helmets
    };

    Ok(Json(response_json))
}


pub fn size_validation(h_size: String) -> bool {
    let mut valid = false;
    if h_size == "XS" || h_size == "S" || h_size == "M" || h_size == "L" || h_size == "XL" || h_size == "XXL"{
        valid = true;
    }
    valid
}

pub fn h_type_validation(h_type: String) -> bool {
    let mut valid = false;
    //'full face', 'enduro', 'trial', 'cross-country'
    if h_type == "full face" || h_type == "enduro" || h_type == "trial" || h_type == "cross-country"{
        valid = true;
    }
    valid
}
#[openapi(tag = "Helmets")]
#[post("/helmets/new", data = "<body>")]
pub async fn create_helmet_handler(
    mut body: Json<Helmet>,
    data: &State<AppState>,
) -> Result<Json<SingleHelmetResponse>, Custom<Json<GenericResponse>>>{
    
    use crate::schema::helmets::dsl::*;
    let connection = &mut establish_connection();
    let vec = helmets
        .load::<Helmet>(connection)
        .expect("Error loading helmets");
    for helmet in vec.iter() {
        if helmet.brand == body.brand && helmet.model==helmet.model && helmet.sold.eq(&false) {
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Helmet already exists DING DONG".to_string(),
            };
            return Err(Custom(Status::BadRequest, Json(response_json)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now().naive_utc();
    body.created_at = datetime;
    body.updated_at = Some(datetime);
    body.sold = false;
    let new_helmet =Helmet{
        h_id:uuid_id.to_string(),
        brand:body.brand.clone(),
        model:body.model.clone(),
        h_type:body.h_type.clone(),
        size:body.size.clone(),
        price:body.price,
        created_at:body.created_at.clone(),
        updated_at: Some(datetime.clone()),
        sold:body.sold
    };
    //check validations
    if !size_validation(new_helmet.size.clone()) || !h_type_validation(new_helmet.h_type.clone()){
        let error = GenericResponse {
            status: "error".to_string(),
            message: "Invalid size or htype".to_string(),
        };
        return Err(Custom(Status::BadRequest, Json(error)));
    }

    let helmet = new_helmet.to_owned(); 
    let helmet_for_db = helmet.clone();

    let connection = &mut establish_connection();
    diesel::insert_into(helmets)
    .values(&helmet_for_db)
    .execute(connection)
    .expect("Error saving new helmet PAOAO");

    let json_response =SingleHelmetResponse{
        status: "success".to_string(),
        helmet: HelmetData {
            helmet: helmet,
        },
    };

    Ok(Json(json_response))
}

#[openapi(tag = "Helmets")]
#[get("/helmets/get/<helm_id>")]
pub async fn get_helmet_handler(
    helm_id: String,
    data: &State<AppState>,
) -> Result<Json<SingleHelmetResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::helmets::dsl::*;
    let connection = &mut establish_connection();
    let helm_id_clone = helm_id.clone();
    let result = helmets
        .find(helm_id_clone)
        .first::<Helmet>(connection)
        .ok();

    match result {
        Some(helmet) => {
            let json_response = SingleHelmetResponse {
                status: "success".to_string(),
                helmet: HelmetData {
                    helmet: helmet.to_owned(),
                },
            };
            return Ok(Json(json_response));
        }
        None => {
            let error = GenericResponse {
                status: "error".to_string(),
                message: "Helmet not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(error)));
        }
    }

}

#[openapi(tag = "Helmets")]
#[post("/helmets/edit/<helm_id>", data = "<body>")]
pub async fn update_helmet_handler(
    helm_id: String,
    body: Json<UpdateHelmet>,
    data: &State<AppState>,
) -> Result<Json<SingleHelmetResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::helmets::dsl::*;
    let connection = &mut establish_connection();
    let helm_id_clone = helm_id.clone();
    let result = helmets
        .find(helm_id_clone)
        .first::<Helmet>(connection)
        .ok();
    
    match result {
        None => {
            let error = GenericResponse {
                status: "error".to_string(),
                message: "Helmet not found".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(error)));
        }
        Some(old_helmet) => {
            let datetime = Utc::now().naive_utc();
            let h_brand = body.brand.to_owned().unwrap_or(old_helmet.brand.to_owned());
            let h_model = body.model.to_owned().unwrap_or(old_helmet.model.to_owned());
            let h_h_type = body.htype.to_owned().unwrap_or(old_helmet.h_type.to_owned());
            let h_size = body.size.to_owned().unwrap_or(old_helmet.size.to_owned());
            
            let payload = Helmet {
            h_id: old_helmet.h_id.to_owned(),
            brand: if !h_brand.is_empty() { 
                h_brand 
            } else {
                old_helmet.brand.to_owned()
            },
            model: if !h_model.is_empty() { 
                h_model 
            } else {
                old_helmet.model.to_owned()
            },
            h_type: if !h_h_type.is_empty() { 
                h_h_type
            } else {
                old_helmet.h_type.to_owned()
            },
            size: if !h_size.is_empty() { 
                h_size 
            } else {
                old_helmet.size.to_owned()
            },
            price: if body.price.is_some() { 
                body.price.unwrap()
            } else {
                old_helmet.price
            },
            sold: if body.sold.is_some() { 
                body.sold.unwrap().clone()
            } else {
                old_helmet.sold
            },
            created_at: old_helmet.created_at.to_owned(),
            updated_at: Some(datetime),
            };  
            //check validations
            if !size_validation(payload.size.clone()) || !h_type_validation(payload.h_type.clone()){
                let error = GenericResponse {
                    status: "error".to_string(),
                    message: "Invalid size or htype".to_string(),
                };
                return Err(Custom(Status::BadRequest, Json(error)));
            }   

            let connection = &mut establish_connection();
            diesel::update(helmets.find(old_helmet.h_id.clone()))
            .set(&payload)
            .execute(connection)
            .expect("Error updating helmet");

        let json_response = SingleHelmetResponse {
            status: "success".to_string(),
            helmet: HelmetData {
                helmet: payload.clone(),
            },
        };
        return Ok(Json(json_response));
        }
        
        
    }
}

pub fn delete_helmet_dependencies(helmid: String,data: &State<AppState>) -> Result<usize, diesel::result::Error> {
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let result =riders 
        .filter(helmet_id.eq(helmid.clone()))
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
    let helm_id_clone = helmid.clone();
    let connection = &mut establish_connection();
    diesel::delete(riders.filter(helmet_id.eq(helm_id_clone))).execute(connection)
}

#[openapi(tag = "Helmets")]
#[post("/helmets/delete/<helm_id>")]
pub async fn delete_helmet_handler(
    helm_id: String,
    data: &State<AppState>,
) -> Result<Json<GenericResponse>, Custom<Json<GenericResponse>>> {
    
    use crate::schema::helmets::dsl::*;
    let connection = &mut establish_connection();
    let helm_id_clone = helm_id.clone();

    /*match diesel::delete(helmets.find(helm_id_clone))
        .execute(connection){
        Ok(_) => {
            let response_json = GenericResponse {
                status: "success".to_string(),
                message: "Helmet deleted".to_string(),
            };

            return Ok(Json(response_json));
        }
        Err(_) => {
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Helmet not found".to_string(),
            };

            return Err(Custom(Status::NotFound, Json(response_json)));
        }
    }*/

    match delete_helmet_dependencies(helm_id.clone(),data){
        Ok(_) => {
            match diesel::delete(helmets.find(helm_id_clone)).execute(connection){
                Ok(_) => {
                    let response_json = GenericResponse {
                    status: "success".to_string(),
                    message: "Helmet deleted".to_string(),
                    };
                    return Ok(Json(response_json));
                }
                Err(_) => {
                    let response_json = GenericResponse {
                        status: "error".to_string(),
                        message: "Helmet not found".to_string(),
                    };
                    return Err(Custom(Status::NotFound, Json(response_json)));
                }
            }
        },
        Err(_) => {
            let response_json = GenericResponse {
                status: "error".to_string(),
                message: "Error at helmet dependency deletion".to_string(),
            };
            return Err(Custom(Status::NotFound, Json(response_json)));
        }
    }

}

pub fn get_no_riders_for_helmet(helmid: String) -> usize {
    
    use crate::schema::riders::dsl::*;
    let connection = &mut establish_connection();
    let helm_id_clone = helmid.clone();
    let result = riders
        .filter(helmet_id.eq(helm_id_clone))
        .load::<Rider>(connection)
        .expect("Error loading riders");
    result.len()
}

#[openapi(tag = "Helmets")]
#[get("/helmets/mostUsed")]
pub async fn get_most_used_helmets_handler(
    data: &State<AppState>,
) -> Result<Json<HelmetStatListResponse>, Custom<Json<GenericResponse>>> {
    use crate::schema::helmets::dsl::*;
    let connection = &mut establish_connection();
    let result = helmets
        .load::<Helmet>(connection)
        .expect("Error loading helmets");

    let mut helm_stats: Vec<HelmetStat> = Vec::new();
    for helmet in result.clone() {
        let no_riders = get_no_riders_for_helmet(helmet.h_id.clone());
        let mut helmet_stat = HelmetStat {
            helmet: helmet.clone(),
            no_riders: no_riders,
        };
        helm_stats.push(helmet_stat);
    }
    helm_stats.sort_by(|a, b| b.no_riders.cmp(&a.no_riders)); 
    
    let json_response = HelmetStatListResponse {
        status: "success".to_string(),
        helmets: helm_stats,
    };

    return Ok(Json(json_response));
}