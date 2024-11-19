use crate::{
    models::post_model::PostModel, schemas::post_schema::{PostSchema,UpdatePost}, services::post_service::{create_post_service, delete_post_service, get_all_posts, get_one_post_service, update_post_services}, AppState
};
use actix_web::{patch,delete,get,post,web,HttpResponse,Responder};
use serde_json::json;



#[get("/healthcheck")]
async fn apihealthcheck()->impl Responder {
    let message : &str = "Build simple crud api with rust in postgres db 🚀🚀";
    HttpResponse::Ok().json(json!({"status":"success","message":message}))
}

#[get("/post")]
pub async fn get_all_post(
    data: web::Data<AppState>,
) -> impl Responder {
    match get_all_posts(data).await {
        Ok(posts) => {
            let json_response = serde_json::json!({
                "status": "ok",
                "data": posts,
            });
            HttpResponse::Ok().json(json_response)
        },
        Err(_) => {
            let message = "Something bad happened when fetching all posts";
            HttpResponse::InternalServerError().json(
                serde_json::json!({"status": "error", "message": message}),
            )
        },
    }
}

#[post("/post")]
async fn create_post_handlers(
    body:web::Json<PostSchema>,
    data:web::Data<AppState>
) -> impl Responder {
    match create_post_service(data,body.into_inner()).await {
        Ok(post)=>{
            let response_json = serde_json::json!({"status":"success","data":serde_json::json!({
                "post":post
            })});

            return HttpResponse::Created().json(json!(response_json))
        }
        Err(e)=>{
            if e.to_string()
            .contains("duplicate key value violates unique constraint")
                {
                    return HttpResponse::BadRequest()
                    .json(serde_json::json!({"status": "fail","message": "Post with that title already exists"}));
                }

                return HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
                }
    }
}

#[get("/post/{id}")]
async fn detail_post(
    path:web::Path<uuid::Uuid>,
    data:web::Data<AppState>
) -> impl Responder {
    let post_id = path.into_inner();
    match get_one_post_service(&data, post_id).await {
        Ok(post)=>{
            let response_json = serde_json::json!({"status":"success","data":serde_json::json!({
                "post":post
            })});

            return HttpResponse::Ok().json(json!(response_json))
        }
        Err(_)=>{
            let message = format!("Note with ID: {} not found", post_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}))
                }
    }
}

#[patch("/post/{id}")]
async fn update_post_controller(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdatePost>,
    data: web::Data<AppState>,
) -> impl Responder {
    let post_id = path.into_inner();

    let old_post = get_one_post_service(&data, post_id).await;

    if old_post.is_err() {
        let message = format!("Post with ID: {} not found", post_id);
        return HttpResponse::NotFound()
            .json(json!({"status": "fail", "message": message}));
    }

    let existing_post:PostModel = old_post.unwrap();
    let title_payload= body.title.clone().unwrap_or(existing_post.title);
    let content_payload= body.content.clone().unwrap_or(existing_post.content);
    let payload  = PostSchema {
        title:title_payload,
        content:content_payload
    };

    // Perform the update
    match  update_post_services(post_id, data, payload).await {
        Ok(post) => {
            let post_response = json!({"status": "success", "post": post});
            return HttpResponse::Ok().json(post_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": message}));
        }
    }
}

#[delete("/post/{id}")]
async fn delete_post(
    path: web::Path<uuid::Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let post_id = path.into_inner();

    if delete_post_service(post_id, data).await == "not found" {
        let message = format!("Note with ID: {} not found", post_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(apihealthcheck)
        .service(get_all_post)
        .service(create_post_handlers)
        .service(detail_post)
        .service(delete_post)
        .service(update_post_controller);

    conf.service(scope);
}

