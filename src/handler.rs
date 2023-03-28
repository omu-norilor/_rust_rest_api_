use crate::{
    model::{Todo, UpdateTodoSchema, InsertableTodo},
    response::{GenericResponse, SingleTodoResponse, TodoListResponse},
    db::establish_connection,
};
use chrono::prelude::*;
use rocket::{
    delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, 
};
use diesel::prelude::*;
use uuid::Uuid;

#[get("/todos?<page>&<limit>")]
pub async fn todos_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
) -> Result<Json<TodoListResponse>, Status> {

    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let res = todos
        .load::<Todo>(connection)
        .expect("Error loading todos");

    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let res2 = res
        .clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    let json_response = TodoListResponse {
        status: "ok".to_string(),
        data: res2,
    };

    Ok(Json(json_response))
}

#[post("/todos", data = "<body>")]
pub async fn create_todo_handler(
    body: Json<InsertableTodo>,
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let res = todos
        .load::<Todo>(connection)
        .expect("Error loading todos");

    for todo in res.iter() {
        if todo.title == body.title {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with title: '{}' already exists", todo.title),
            };
            return Err(Custom(Status::Conflict, Json(error_response)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now().naive_utc();

    let new_todo = Todo {
        tid: uuid_id.to_string(),
        title: body.title.clone(),
        content: body.content.clone(),
        completed: false,
        created_at: datetime.clone(),
        updated_at: Some(datetime.clone()),
    };


    let todo = new_todo.to_owned();

    let todo_for_db = todo.clone();
    let connection = &mut establish_connection();
    diesel::insert_into(todos)
        .values(&todo_for_db)
        .execute(connection)
        .expect("Error saving new todo");

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: todo.clone(),
    };

    Ok(Json(json_response))
}

#[get("/todos/<id>")]
pub async fn get_todo_handler(
    id: String,
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {

    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();

    let res = todos
            .find(id.clone())
            .first::<Todo>(connection)
            .ok();

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };

    match res {
        Some(todo) => {
            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: todo,
            };
            Ok(Json(json_response))
        }
        None => Err(Custom(Status::NotFound, Json(error_response))),
    }
}

#[patch("/todos/<id>", data = "<body>")]
pub async fn edit_todo_handler(
    id: String,
    body: Json<UpdateTodoSchema>,
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let res = todos
            .find(id.clone())
            .first::<Todo>(connection)
            .ok();

    match res{
        None => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with ID: {} not found", id),
            };
            return Err(Custom(Status::NotFound, Json(error_response)));
        }
        Some(old_todo) => {
            let time = Utc::now().naive_utc();
            let payload = Todo {
                tid: id.clone(),
                title: body.title.to_owned().unwrap_or(old_todo.title.clone()),
                completed: body.completed.to_owned().unwrap_or(false),
                content: body.content.to_owned().unwrap_or(old_todo.content.clone()),
                created_at: old_todo.created_at.clone(),
                updated_at: Some(time.clone())
            };

            let connection = &mut establish_connection();

            match diesel::update(todos.find(id.clone()))
                .set(&payload)
                .execute(connection){
                    Ok(_) => {
                        let json_response = SingleTodoResponse {
                            status: "success".to_string(),
                            data: payload.clone(),
                        };
                        return Ok(Json(json_response));
                    },
                    Err(_) => {
                        let error_response = GenericResponse {
                            status: "fail".to_string(),
                            message: format!("Todo with ID: {} not found", id),
                        };
                        return Err(Custom(Status::NotFound, Json(error_response)));
                    }
                }
        }
    }
    
}

#[delete("/todos/<id>")]
pub async fn delete_todo_handler(
    id: String,
) -> Result<Status, Custom<Json<GenericResponse>>> {
    
    use crate::schema::todos::dsl::*;

    let connection = &mut establish_connection();

    match diesel::delete(todos.find(id.clone())).execute(connection) {
        Ok(_) => {
            return Ok(Status::Ok);
        }
        Err(_) => {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with ID: {} not found", id),
            };
            return Err(Custom(Status::NotFound, Json(error_response)));
        }
    }
}
