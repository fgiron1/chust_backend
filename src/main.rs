mod service;

use warp::Filter;
use crate::service::model;

#[tokio::main]
async fn main() {

    // Basic endpoints

    let root = warp::path::end().map(|| "You're in /\nNice");
    let login = warp::path!("login").map(|| "Trying to log in eh?");
    let sign_up = warp::path!("signup").map(|| "Trying to sign up eh?");

    // Rooms endpoints
    
    // GET /room/12
    // EXAMPLE REQUEST TO ESTABLISH VERB PER-REQUEST
    let get_room_by_id = warp::get()
        .and(warp::path!("room" / String))
        .map(|room_id| format!("Trying to access room {room_id}'s data eh?"));
    
    /*

    POST /room 
    Private room
        {
            creator_id: 32fbc5f5-7613-4f11-b6c6-fefa704797d8 (UUIDv4),
            password: 2624b7398b668fbf22b33e9f8b574dc48b60e75c33915452a61c1fe8a357642c (contrasena123 SHA2-256)
            salt: pimienta
        }
    Public room
        {
            creator_id: 32fbc5f5-7613-4f11-b6c6-fefa704797d8,
        }

    COMPRUEBA si mandas un body con un creator_id, sin password ni salt, si te lo acepta o no.

    */
    let post_room = warp::post()
    .and(warp::path!("room"))
    .and(warp::path::end())
    .and(warp::body::json())
    .map(|mut room : model::Room| {
        warp::reply::json(&room);
    });


    let routes = warp::any().and(
        root
            .or(get_room_by_id)
            .or(post_room)
            .or(login)
            .or(sign_up)
    );

    warp::serve(routes)
    .run(([127, 0, 0, 1], 8080))
    .await;

}