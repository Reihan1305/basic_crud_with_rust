use crate::models::post_model::PostModel;
use crate::schemas::post_schema::PostSchema;
use crate::AppState;
use actix_web::{web};
use sqlx::{query, query_as, Error};
use uuid::Uuid;

pub async fn get_all_posts(data: web::Data<AppState>) -> Result<Vec<PostModel>, Error> {
    let get_all_query = query_as!(PostModel, "SELECT * FROM post ORDER BY id")
        .fetch_all(&data.db)
        .await;

    get_all_query
}

pub async fn create_post_service(
    data: web::Data<AppState>,
    new_post: PostSchema,
) -> Result<PostModel, Error> {
    let create_post = sqlx::query_as!(
        PostModel,
        "INSERT INTO post(title, content) VALUES ($1, $2) RETURNING *",
        new_post.title,
        new_post.content,
    )
    .fetch_one(&data.db)
    .await;

    create_post
}

pub async fn get_one_post_service(data:&AppState,id:Uuid) -> Result<PostModel,Error> {
    let get_one_post = query_as!(
        PostModel,
        "SELECT * FROM post WHERE id=$1",id
    )
    .fetch_one(&data.db)
    .await;

    get_one_post
}

pub async fn update_post_services(id:Uuid,data:web::Data<AppState>,update_body:PostSchema) -> Result<PostModel,Error> {
    let update_post = query_as!(
        PostModel,
        "UPDATE post SET title = $1, content = $2 WHERE id = $3 RETURNING *",
        update_body.title,
        update_body.content,
        id
    )
    .fetch_one(&data.db)
    .await;

    update_post
}

pub async fn delete_post_service(id:Uuid,data:web::Data<AppState>) ->String {
    let delete_query = query!("DELETE FROM post  WHERE id = $1", id)
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if delete_query == 0 {
        let error_messsage:String = String::from("not found");
        return error_messsage 
    }
    let success_message:String = String::from("Success");
    success_message
}