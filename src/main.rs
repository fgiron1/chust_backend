mod service;

use service::routes::set_endpoints;

#[tokio::main]
async fn main() {

    let routes = set_endpoints();
    
    warp::serve(routes)
    .run(([127, 0, 0, 1], 8080))
    .await;

}