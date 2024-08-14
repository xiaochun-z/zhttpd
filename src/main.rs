use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Serve files from the assets directory
    let routes = warp::fs::dir("./assets");

    // Try to bind to port 8000
    let port = match TcpListener::bind("127.0.0.1:8000").await {
        Ok(listener) => listener.local_addr().unwrap().port(),
        Err(_) => {
            // If port 8000 is unavailable, bind to a random port
            TcpListener::bind("127.0.0.1:0")
                .await
                .unwrap()
                .local_addr()
                .unwrap()
                .port()
        }
    };

    println!(
        "Serving files from the current directory on http://localhost:{}",
        port
    );

    #[cfg(any(target_os = "windows"))] //, target_os = "macos"
    {
        // Open browser
        open::that(format!("http://localhost:{}", port)).unwrap();
    }

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
