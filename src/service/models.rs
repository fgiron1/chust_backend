use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Room {

    id : Option<Uuid>,
    creator_id : Uuid,
    password : Option<String>, // Internamente se maneja el hash.
    salt : Option<String>,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id : Uuid,
    name : String,
}

#[derive(Debug)]
pub struct ErrorInput;
impl warp::reject::Reject for ErrorInput {}
