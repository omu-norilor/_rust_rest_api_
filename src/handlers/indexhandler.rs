use crate::response::GenericResponse;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

use rocket::{
     get, http::Status,  serde::json::Json
};


#[openapi(skip)]
#[get("/")]
pub async fn index_handler() -> Result<Json<GenericResponse>, Status>  {
    const msg: &str = "Oh helh yeah boaeh weh goth it runnin like a horsey!";
    
    let response_json = GenericResponse {
        status: "success".to_string(),
        message: msg.to_string(),
    };
    
    Ok(Json(response_json))
}
