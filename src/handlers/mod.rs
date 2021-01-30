use actix_web::{web,  HttpResponse};
use chrono::prelude::*;
use crate::dto::{SaveTodoRequest, SaveTodoResponse, GetAllRequest, GetAllResponse, UpdateResponse, DeleteResponse};
use crate::model::Todo;
use uuid::Uuid;
use bson::{doc};
use crate::repository::Repository;

#[derive(Clone)]
pub struct HttpContext {
    pub logger : slog::Logger,
    pub repository: Repository,
}


pub async fn index() -> HttpResponse {
    //todo use swagger
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body("<html><head><title>ToDo Rust Api</title></head><body>\
    <h1>Welcome to ToDo app api which is written in rust language.</h1>\
    <ul style='font-size:30px'>\
    <li><strong>Path:</strong> /todos <strong>Methods:</strong> GET & POST</li>\
    <li><strong>Path:</strong> /todos/{id} <strong>Methods:</strong> Delete & Get </li>\
    <li><strong>Path:</strong>/todos/{id}/done <strong>Methods:</strong> Patch</li> \
    </ul>\
    </body></html>")
}

pub async fn save(context: web::Data<HttpContext>, request: web::Json<SaveTodoRequest>) -> HttpResponse {
    let logger = context.logger.clone();
    let todo_request = request.into_inner();
    if todo_request.name.is_empty() {
        return HttpResponse::BadRequest().finish()
    }
    let id = Uuid::new_v4().to_string();
    let todo = Todo {
        added_at: Utc::now(),
        name: todo_request.name,
        done: false,
        tags: todo_request.tags,
        id: id.to_owned(),
    };

    let result = context.repository.insert(todo).await;
    match result {
        Ok(result) => {
            info!(logger, "Saved");
            HttpResponse::Ok().json(SaveTodoResponse{success:result, id})
        },
        Err(e) => {
            error!(logger,"Error while saving, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get(context: web::Data<HttpContext>, id: web::Path<String>) -> HttpResponse {
    let logger = context.logger.clone();
    if id.is_empty() {
        return HttpResponse::BadRequest().finish()
    }

    let result = context.repository.get(&id.into_inner()).await;
    match result {
        Ok(result) => {
            match result {
                Some(r) => HttpResponse::Ok().json(r),
                None => HttpResponse::NoContent().finish(),
            }
        },
        Err(e) => {
            error!(logger,"Error while getting one, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_all(context: web::Data<HttpContext>, request: web::Query<GetAllRequest>) -> HttpResponse {
    let logger = context.logger.clone();
    let result = context.repository.get_all(request.into_inner().done).await;
    match result {
        Ok(result) => {
            if !result.is_empty() {
                let response = GetAllResponse{
                    todos: result
                };
                HttpResponse::Ok().json(response)
            }else {
                HttpResponse::NoContent().finish()
            }
        },
        Err(e) => {
            error!(logger,"Error while getting all, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub async fn toggle_done(context: web::Data<HttpContext>, id: web::Path<String>) -> HttpResponse {
    let logger = context.logger.clone();
    if id.is_empty() {
        return HttpResponse::BadRequest().finish()
    }
    let id = id.into_inner();


    let get_result = context.repository.get(id.as_str()).await;
    match get_result {
        Ok(get_result) => {
            match get_result {
                Some(todo) => {
                    //todo bson leak
                    let update_doc = doc!{"$set" : {"done":!todo.done}};
                    let update_result = context.repository.update(id.as_str(), update_doc).await;
                    match update_result {
                        Ok(success) => {
                            HttpResponse::Ok().json(UpdateResponse{success})
                        }
                        Err(e) => {
                            error!(logger,"Error while updating, {:?}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                },
                None => HttpResponse::BadRequest().finish(),
            }
        },
        Err(e) => {
            error!(logger,"Error while updating, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete(context: web::Data<HttpContext>, id: web::Path<String>) -> HttpResponse {
    let logger = context.logger.clone();
    info!(logger, "herqqwe");
    if id.is_empty() {
        return HttpResponse::BadRequest().finish()
    }
    let id = id.into_inner();

    let result = context.repository.delete(id.as_str()).await;
    match result {
        Ok(result) => {
            info!(logger, "deleted");
            HttpResponse::Ok().json(DeleteResponse{success:result})
        },
        Err(e) => {
            error!(logger,"Error while deleting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}