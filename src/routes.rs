use crate::db::MainDatabase;
use crate::models::Dragon;
use mongodb::bson::doc;
use rocket::{
    http::Status, post, response::status,
    serde::json::Json,
};
use rocket_db_pools::Connection;
use serde_json::{json, Value};

#[post("/api/dragons", data = "<data>", format = "json")]
pub async fn add_dragon(
    db: Connection<MainDatabase>,
    data: Json<Dragon>,
) -> status::Custom<Json<Value>> {
    match db
        .database("game_of_thrones")
        .collection::<Dragon>("dragons")
        .insert_one(data.into_inner(), None)
        .await
    {
        Ok(res) => {
            if let Some(id) = res.inserted_id.as_object_id() {
                status::Custom(
                    Status::Created,
                    Json(json!({"status": "success", "message": format!("Dragon ({}) created successfully", id.to_string())})),
                )
            } else {
                status::Custom(
                    Status::InternalServerError,
                    Json(json!({"status": "error", "message": "Failed to retrieve inserted ID"})),
                )
            }
        }
        Err(e) => {
            let error_message = format!("Dragon could not be created: {}", e);
            status::Custom(
                Status::BadRequest,
                Json(json!({"status": "error", "message": error_message})),
            )
        }
    }
}