use std::{convert::Infallible};
use super::models;
use warp::{hyper::StatusCode};

pub fn signup(pwd_encrypted : String, ){

}

pub async fn save_room(
    room : models::Room
) -> Result<impl warp::Reply, warp::Rejection> {
    // Business logic. If not satisfied, rejected.
    if valid_room(&room) {
        // Save to mongoDB
        return Ok(warp::reply::json(&room));
    } else {
        Err(warp::reject::custom(models::ErrorInput))
    }
}
    
// SYNCHRONOUS VALIDATION AGAINST BUSINESS RULES
// NOT MEANT FOR ANY DATABASE VALIDATION (WOULD BE ASYNC)
fn valid_room(model : &models::Room) -> bool {
    // Validation logic
    return true;
}
// Aquí https://morioh.com/p/b7903b749bcb mandan como respuesta un JSON porque su error custom de reject tiene campos.
// Hemos decidido simplificarlo en esta implementación y no tiene ningun campo, por lo que cuando se de
// simplemente contestamos con un código de estado 404 y decimos NOT FOUND
pub async fn handle_rejection(err: warp::Rejection) -> std::result::Result<impl warp::Reply, Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if let Some(e) = err.find::<models::ErrorInput>() {
        Ok(warp::reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
    }
}