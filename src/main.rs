#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod handlers;
mod model;
mod response;
mod db;
mod schema;

use handlers::indexhandler::*;

use handlers::bikehandler::*;
 
use handlers::helmhandler::*;

use handlers::riderhandler::*;

use handlers::eventhandler::*;

use handlers::eventriderhandler::*;



// use rocket::{get, http::Status, serde::json::Json};
// use serde::Serialize;
// use rocket_okapi::settings;
// use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
// use rocket::fairing::{Fairing, Info, Kind};

use rocket::{get, http::Status, http::Header, serde::json::Json, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use rocket_okapi::*;


#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    let app_data = model::AppState::init();
    rocket::build().attach(CORS).manage(app_data).mount(
        "/", 
        openapi_get_routes![
            index_handler,
            bikes_list_handler,
            create_bike_handler,
            get_bike_handler,
            update_bike_handler,
            delete_bike_handler,
            bikes_filter_handler,
            create_more_bikes_handler,
            create_helmet_handler,
            delete_helmet_handler,
            update_helmet_handler,
            get_helmet_handler,
            helmets_list_handler,
            get_most_used_helmets_handler,
            create_rider_handler,
            delete_rider_handler,
            update_rider_handler,
            get_rider_handler,
            riders_list_handler,
            get_most_active_riders_handler,
            create_event_handler,
            delete_event_handler,
            update_event_handler,
            get_event_handler,
            events_list_handler,
            create_eventrider_handler,
            delete_eventrider_handler,
            update_eventrider_handler,
            get_eventrider_handler,
            eventrider_list_handler
        ],
    )
    .mount(
        "/swagger-ui/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
    .mount(
        "/rapidoc/",
        make_rapidoc(&RapiDocConfig {
            general: GeneralConfig {
                spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                ..Default::default()
            },
            hide_show: HideShowConfig {
                allow_spec_url_load: false,
                allow_spec_file_load: false,
                ..Default::default()
            },
            ..Default::default()
        }),
    )
}

#[cfg(test)]
mod test{
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use crate::response::RiderStatListResponse;
    use crate::response::HelmetStatListResponse;

    #[test]fn test_active_riders() {

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.get("/riders/mostActive").dispatch();
        assert_eq!(response.status(), Status::Ok);
        
        let body: RiderStatListResponse =response.into_json::<RiderStatListResponse>().expect("valid json");
        let data = body.riders;
        
        let rider_activity = data.iter().map(|x| x.no_events).collect::<Vec<usize>>(); 
        for i in 0..rider_activity.len()-1 {
            assert!(rider_activity[i] >= rider_activity[i+1]);
        }
        
    }

    #[test]
    fn test_most_used_helmets() {

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.get("/helmets/mostUsed").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let body: HelmetStatListResponse =response.into_json::<HelmetStatListResponse>().expect("valid json");
        let data = body.helmets;
        
        let helmet_uses = data.iter().map(|x| x.no_riders).collect::<Vec<usize>>();
        for i in 0..helmet_uses.len()-1 {
            assert!(helmet_uses[i] >= helmet_uses[i+1]);
        }
    }
}