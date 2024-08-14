use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Serve files from the directory
    let www_root = "./public_html";
    let routes = warp::fs::dir(www_root);

    // Try to bind to port 8000
    let port = match TcpListener::bind("0.0.0.0:8000").await {
        Ok(listener) => listener.local_addr().unwrap().port(),
        Err(_) => {
            // If port 8000 is unavailable, bind to a random port
            TcpListener::bind("0.0.0.0:0")
                .await
                .unwrap()
                .local_addr()
                .unwrap()
                .port()
        }
    };

    println!(
        "Serving '{0}' directory on http://localhost:{1}\nPut your HTML/CSS/JavaScript files to '{0}' folder, if you don't have this folder, you can create it by yourself.\n\nPress Ctrl+C to quit...",
        www_root, port
    );

    #[cfg(any(target_os = "windows"))] //, target_os = "macos"
    {
        // Open browser
        open::that(format!("http://localhost:{}", port)).unwrap();
    }

    let server_addr = ([0, 0, 0, 0], port);

    // Start the server
    warp::serve(routes).run(server_addr).await;
}
