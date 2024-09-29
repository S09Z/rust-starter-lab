use warp::Filter;
use log::info;

#[tokio::main]
async fn main() {
    // Log initialization
    env_logger::init();

    // Define WebSocket route
    let ws_route = warp::ws().and_then(handle_ws);

    // Define routes
    let routes = warp::path("api")
        .and(warp::path("chat"))
        .and(ws_route)
        .or(warp::path("api").and(warp::path("status")).map(|| warp::reply::json(&"Chat API is running!")));

    // Start the server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

// Handle WebSocket connections
async fn handle_ws(ws: warp::ws::Ws) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(ws.on_upgrade(|_websocket| async {
        // Handle WebSocket connection
        info!("New WebSocket connection established");
        // Add WebSocket handling logic here
    }))
}
