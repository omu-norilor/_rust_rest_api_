use crate::db::establish_connection;
use crate::model::*;
use chrono::prelude::*;
use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json,
    serde::Deserialize
};
use diesel::prelude::*;
use uuid::Uuid;
use rocket::serde::json::from_str;

#[delete("/data")]
// Delete all data from all tables
// This is for testing purposes only
pub fn delete_all_data() -> Custom<String> {

    use crate::schema::bikes::dsl::*;
    use crate::schema::helmets::dsl::*;
    use crate::schema::riders::dsl::*;
    use crate::schema::events::dsl::*;
    use crate::schema::eventrider::dsl::*;

    let conn = &mut establish_connection();
    let mut result = String::new();
    match diesel::delete(bikes).execute(conn){
        Ok(_) => result.push_str("Deleted all data from bikes table\n"),
        Err(_) => result.push_str("Failed to delete data from bikes table\n"),
    };
    match diesel::delete(helmets).execute(conn){
        Ok(_) => result.push_str("Deleted all data from helmets table\n"),
        Err(_) => result.push_str("Failed to delete data from helmets table\n"),
    };
    match diesel::delete(riders).execute(conn){
        Ok(_) => result.push_str("Deleted all data from riders table\n"),
        Err(_) => result.push_str("Failed to delete data from riders table\n"),
    };
    match diesel::delete(events).execute(conn){
        Ok(_) => result.push_str("Deleted all data from events table\n"),
        Err(_) => result.push_str("Failed to delete data from events table\n"),
    };
    match diesel::delete(eventrider).execute(conn){
        Ok(_) => result.push_str("Deleted all data from eventrider table\n"),
        Err(_) => result.push_str("Failed to delete data from eventrider table"),
    }
    Custom(Status::Ok, result)
}

pub fn initialize_data_riders() ->String{
    use crate::schema::riders::dsl::*;

    let json_riders = r#"[
    {
    "name" : "Alice Smith",
    "email" : "alice.smith@gmail.com",
    "password" : "P@ssw0rd!"
    },

    {
    "name" : "Bob Johnson",
    "email" : "bob.johnson@yahoo.com",
    "password" : "MyP@ssw0rd!"
    },

    {
    "name" : "Sara Lee",
    "email" : "sara.lee@hotmail.com",
    "password" : "CakeIsYummy"
    },

    {
    "name" : "Alex Lee",
    "email" : "alex.lee@gmail.com",
    "password" : "IloveM@ths"
    },

    {
    "name" : "David Chen",
    "email" : "david.chen@gmail.com",
    "password" : "qwerty123"
    },

    {
    "name" : "Emily Davis",
    "email" : "emily.davis@yahoo.com",
    "password" : "MyDogIsCute!"
    },

    {
    "name" : "Grace Wang",
    "email" : "grace.wang@hotmail.com",
    "password" : "Summer2022"
    },

    {
    "name" : "Jack Brown",
    "email" : "jack.brown@gmail.com",
    "password" : "Football10"
    },

    {
    "name" : "Lily Jones",
    "email" : "lily.jones@yahoo.com",
    "password" : "Sunflowers123"
    },

    {
    "name" : "Michael Lee",
    "email" : "michael.lee@hotmail.com",
    "password" : "BeyonceFan1"
    }
    ]"#;

    #[derive(Deserialize)]
    struct RiderNoId {
        name: String,
        email: String,
        phone: String,
    }

    let conn = &mut establish_connection();
    let temp_riders = from_str::<Vec<RiderNoId>>(json_riders).unwrap();
    let other_riders: Vec<Rider> = temp_riders.iter().map(|rider| rider {
        uid: Uuid::new_v4().to_string(),
        name: rider.name.clone(),
        email: rider.email.clone(),
        password: rider.password.clone(),
        created_at: Utc::now().naive_utc(),
        }).collect();
    let mut result = String::new();

    match diesel::insert_into(riders)
        .values(&other_riders)
        .execute(conn){
            Ok(_) => result.push_str("riders inserted successfully"),
            Err(e) => result.push_str(&format!("Error inserting riders: {}", e)),
        };

    result
}

pub fn initialize_data_bikes() -> String{

    use crate::schema::bikes::dsl::*;

    
    let json_bikes = r#"[

    {
    "title": "Buy groceries",
    "content": "Get milk, bread, eggs, cheese, and vegetables from the supermarket"
    },

    {
    "title": "Clean the bathroom",
    "content": "Scrub the toilet, sink, and shower. Sweep and mop the floor"
    },

    {
    "title": "Go for a run",
    "content": "Jog for 30 minutes around the park. Stretch before and after"
    },

    {
    "title": "Call mom",
    "content": "Ask her how she's doing and tell her about your week"
    },

    {
    "title": "Finish report",
    "content": "Write the conclusion section and proofread the entire document"
    },

    {
    "title": "Organize closet",
    "content": "Sort clothes by season, donate or sell items you no longer wear"
    },

    {
    "title": "Pay bills",
    "content": "Pay rent, electricity, water, and internet bills before the due date"
    },

    {
    "title": "Read book",
    "content": "Read two chapters of 'The Great Gatsby' before going to bed"
    },

    {
    "title": "Cook dinner",
    "content": "Make chicken stir-fry with rice and vegetables. Follow recipe instructions"
    },

    {
    "title": "Study for exam",
    "content": "Review notes, do practice questions, and memorize key concepts for biology exam"
    }
    ]"#;
    /*

    #[derive(Deserialize)]
    struct RiderNoId {
        name: String,
        email: String,
        password: String,
    }

    let conn = &mut establish_connection();
    let temp_riders = from_str::<Vec<RiderNoId>>(json_riders).unwrap();
    let other_riders: Vec<rider> = temp_riders.iter().map(|rider| rider {
        uid: Uuid::new_v4().to_string(),
        name: rider.name.clone(),
        email: rider.email.clone(),
        password: rider.password.clone(),
        created_at: Utc::now().naive_utc(),
        }).collect();
    */
    
    #[derive(Deserialize)]
    struct BikeNoId {
        brand: String,
        name: String,
    }
    let conn = &mut establish_connection();
    let temp_bikes = from_str::<Vec<BikeNoId>>(json_bikes).unwrap();
    let other_bikes: Vec<Bike> = temp_bikes.iter().map(|bike| bike {
        tid: Uuid::new_v4().to_string(),
        title: bike.title.clone(),
        content: bike.content.clone(),
        created_at: Utc::now().naive_utc(),
        completed: false,
        updated_at: None,
        }).collect();
    let mut result = String::new();
    match diesel::insert_into(bikes)
        .values(&other_bikes)
        .execute(conn){
            Ok(_) => result.push_str("bikes inserted successfully"),
            Err(e) => result.push_str(&format!("Error inserting bikes: {}", e)),
        };

    result
}

pub fn initialize_data_helmets() -> String {

    use crate::schema::helmets::dsl::*;
    let json_helmets = r#"[

    {
    "title": "Work",
    "priority": 9
    },

    {
    "title": "Household Chores",
    "priority": 7
    },

    {
    "title": "Fitness",
    "priority": 6
    },

    {
    "title": "Errands",
    "priority": 5
    },

    {
    "title": "Family",
    "priority": 8
    },

    {
    "title": "Self-Improvement",
    "priority": 7
    },

    {
    "title": "Social Life",
    "priority": 6
    },

    {
    "title": "Finances",
    "priority": 8
    },

    {
    "title": "Travel",
    "priority": 5
    },

    {
    "title": "Volunteering",
    "priority": 7
    }
    ]"#;
    
    fn get_rider_ids() -> Vec<String> {
        use crate::schema::riders::dsl::*;
        let conn = &mut establish_connection();
        let results = riders.load::<rider>(conn).expect("Error loading riders");
        results.iter().map(|rider| rider.uid.clone()).collect()
    }

    let first_rider_id = get_rider_ids()[0].clone();

    #[derive(Deserialize)]
    struct bikeListNoId {
        title: String,
        priority: i32,
    }
    let temp_helmets = from_str::<Vec<bikeListNoId>>(json_helmets).unwrap();
    let other_helmets: Vec<bikeList> = temp_helmets.iter().map(|bikelist| bikeList {
        uid: first_rider_id.clone(),
        tlid: Uuid::new_v4().to_string(),
        title: bikelist.title.clone(),
        priority: bikelist.priority,
        created_at: Utc::now().naive_utc(),
        }).collect();
    let conn = &mut establish_connection();
    let mut result = String::new();
    match diesel::insert_into(helmets)
        .values(&other_helmets)
        .execute(conn){
            Ok(_) => result.push_str("helmets inserted successfully"),
            Err(e) => result.push_str(&format!("Error inserting helmets: {}", e)),
        };

    result
}

pub fn initialize_data_bikeshelmets() -> String{

    fn get_bike_ids() -> Vec<String>{
        use crate::schema::bikes::dsl::*;
        let conn = &mut establish_connection();
        let result = bikes.select(tid).load::<String>(conn).unwrap();
        result
    }

    fn get_bikelist_ids() -> Vec<String>{
        use crate::schema::helmets::dsl::*;
        let conn = &mut establish_connection();
        let result = helmets.select(tlid).load::<String>(conn).unwrap();
        result
    }

    use crate::schema::bikeshelmets::dsl::*;

    let conn = &mut establish_connection();
    let mut result = String::new();

    let bike_ids = get_bike_ids();
    let bikelist_ids = get_bikelist_ids();

    //put first 5 bikes in 1st bikelist, next 5 in 2nd bikelist
    for i in 0..5{
        match diesel::insert_into(bikeshelmets)
            .values((tid.eq(bike_ids[i].clone()), tlid.eq(bikelist_ids[0].clone())))
            .execute(conn){
                Ok(_) => result.push_str("bikelist-bike inserted successfully"),
                Err(e) => result.push_str(&format!("Error inserting bikelist-bike: {}", e)),
            };
    }

    for i in 5..10{
        match diesel::insert_into(bikeshelmets)
            .values((tid.eq(bike_ids[i].clone()), tlid.eq(bikelist_ids[1].clone())))
            .execute(conn){
                Ok(_) => result.push_str("bikelist-bike inserted successfully"),
                Err(e) => result.push_str(&format!("Error inserting bikelist-bike: {}", e)),
            };
    }
    
    result
}

#[post("/data")]
//initialize database with some data
//This is for testing purposes only
pub fn initialize_data() -> Custom<String> {
    let mut result = String::new();
    result.push_str(&initialize_data_riders());
    result.push_str(&initialize_data_bikes());
    result.push_str(&initialize_data_helmets());
    result.push_str(&initialize_data_bikeshelmets());

    Custom(Status::Ok, result)
}



