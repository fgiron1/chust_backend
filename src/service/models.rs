use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Room {

    id : Uuid,
    creator_id : Uuid,
    password : String, // Internamente se maneja el hash.
    salt : String,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    id : Uuid,
    name : String,
}

#[derive(Debug)]
pub struct ErrorInput;
impl warp::reject::Reject for ErrorInput {}
