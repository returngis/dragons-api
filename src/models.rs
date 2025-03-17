use mongodb::bson::oid::ObjectId; 
use rocket::serde::{Deserialize, Serialize}; 

#[derive(Debug, Serialize, Deserialize)] 
#[serde(crate = "rocket::serde")] 
pub struct Dragon { 
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] 
    pub id: Option<ObjectId>, 
    pub name: String, 
    pub owner: String    
}