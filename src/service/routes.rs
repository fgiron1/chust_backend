use warp::Filter;

use super::handlers;


pub fn set_endpoints() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone { 

    // Basic endpoints

    let root = warp::path::end()
        .and(warp::get())
        .map(|| "You're in /\nNice");
    let login = warp::path!("login")
        .and(warp::path::end())
        .and(warp::post())
        .map(|| "Trying to log in eh?");
    let sign_up = warp::path!("signup")
        .and(warp::path::end())
        .and(warp::post())
        .map(|| "Trying to sign up eh?");

    // Rooms endpoints
    
    // GET /room/12 
    let get_room_by_id = warp::path!("room" / String)
        .and(warp::get())
        .map(|room_id: String| format!("Trying to access room {room_id}'s data eh?"));
    
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
    let post_room = warp::path!("room")
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::save_room);

        
    let routes = root
        .or(login)
        .or(sign_up)
        .or(get_room_by_id)
        .or(post_room);

    // Am I returning the newly added behavior? I think not
    routes.recover(handlers::handle_rejection);

    return routes;

}