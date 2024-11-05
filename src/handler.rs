use crate::{
    model::PostModel,
    schema::{UpdatePost,CreatePost},
    AppState,
};
use actix_web::{patch,delete,get,post,web,HttpResponse,Responder};
use serde_json::json;
use sqlx::{query_as,query};

#[get("/healthcheck")]
async fn apihealthcheck()->impl Responder {
    let message : &str = "Build simple crud api with rust in postgres db 🚀🚀";
    HttpResponse::Ok().json(json!({"status":"success","message":message}))
}


#[get("/post")]
pub async fn get_all_post(
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = query_as!(
        PostModel,
        "SELECT * FROM post order by id"
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err(){
        let message :&str = "something bad when fetch all post";
        return HttpResponse::InternalServerError().json(
            json!({"status":"error","message":message})
        )
    }

    let post = query_result.unwrap();

    let json_response = serde_json::json!({
        "status":"ok",
        "data":post
    });

    return HttpResponse::Ok().json(json!(json_response))
}


#[post("/post")]
async fn create_post(
    body:web::Json<CreatePost>,
    data:web::Data<AppState>
) -> impl Responder {
    let query_result = query_as!(
        PostModel,
        "INSERT INTO post(title,content) values ($1, $2) RETURNING *",
        body.title.to_string(),
        body.content.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
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
                    .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
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
    let query_result = query_as!(
        PostModel,
        "SELECT * FROM post WHERE id=$1",post_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
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
async fn update_post(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdatePost>,
    data: web::Data<AppState>,
) -> impl Responder {
    let post_id = path.into_inner();

    // Fetch the existing post first
    let query_result = query_as!(
        PostModel,
        "SELECT * FROM post WHERE id = $1",
        post_id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("Post with ID: {} not found", post_id);
        return HttpResponse::NotFound()
            .json(json!({"status": "fail", "message": message}));
    }

    let existing_post = query_result.unwrap();

    // Prepare the update values
    let title = body.title.clone().unwrap_or(existing_post.title);
    let content = body.content.clone().unwrap_or(existing_post.content);

    // Perform the update
    let update_result = query_as!(
        PostModel,
        "UPDATE post SET title = $1, content = $2 WHERE id = $3 RETURNING *",
        title,
        content,
        post_id
    )
    .fetch_one(&data.db)
    .await;

    match update_result {
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
    let rows_affected = query!("DELETE FROM post  WHERE id = $1", post_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Note with ID: {} not found", post_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(apihealthcheck)
        .service(get_all_post)
        .service(create_post)
        .service(detail_post)
        .service(delete_post)
        .service(update_post);

    conf.service(scope);
}

