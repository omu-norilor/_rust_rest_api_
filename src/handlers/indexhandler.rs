use crate::response::GenericResponse;



use rocket::{
     get, http::Status,  serde::json::Json
};



#[get("/")]
pub async fn index_handler() -> Result<Json<GenericResponse>, Status>  {
    const msg: &str = "Oh helh yeah boaeh weh goth it runnin like a horsey!";
    
    let response_json = GenericResponse {
        status: "success".to_string(),
        message: msg.to_string(),
    };
    
    Ok(Json(response_json))
}
