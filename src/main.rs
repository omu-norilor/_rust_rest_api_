#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod handlers;
mod model;
mod response;
mod db;
mod schema;

use handlers::indexhandler::{
    index_handler
 };

use handlers::bikehandler::{
    create_bike_handler,
    delete_bike_handler,
    update_bike_handler,
    get_bike_handler,
    bikes_list_handler,
    bikes_filter_handler,
    create_more_bikes_handler
};
 
use handlers::helmhandler::{
   create_helmet_handler,
   delete_helmet_handler,
   update_helmet_handler,
   get_helmet_handler,
   helmets_list_handler,
   get_most_used_helmets_handler
};

use handlers::riderhandler::{
    create_rider_handler,
    delete_rider_handler,
    update_rider_handler,
    get_rider_handler,
    riders_list_handler,
    get_most_active_riders_handler
};

use handlers::eventhandler::{
    create_event_handler,
    delete_event_handler,
    update_event_handler,
    get_event_handler,
    events_list_handler
};

use handlers::eventriderhandler::{
    create_eventrider_handler,
    delete_eventrider_handler,
    update_eventrider_handler,
    get_eventrider_handler,
    eventrider_list_handler
};



use rocket::{get, http::Status, serde::json::Json};
use serde::Serialize;
#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}



#[launch]
fn rocket() -> _ {
    let app_data = model::AppState::init();
    rocket::build().manage(app_data).mount("/api", routes![
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
        
    ])
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
        let mut response = client.get("/api/riders/mostActive").dispatch();
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
        let mut response = client.get("/api/helmets/mostUsed").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let body: HelmetStatListResponse =response.into_json::<HelmetStatListResponse>().expect("valid json");
        let data = body.helmets;
        
        let helmet_uses = data.iter().map(|x| x.no_riders).collect::<Vec<usize>>();
        for i in 0..helmet_uses.len()-1 {
            assert!(helmet_uses[i] >= helmet_uses[i+1]);
        }
    }
}