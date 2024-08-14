use get_if_addrs::{get_if_addrs, IfAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Serve files from the assets directory
    let routes = warp::fs::dir("./assets");

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

    let ip = get_if_addrs()
        .unwrap()
        .into_iter()
        .find(|iface| {
            !iface.is_loopback()
                && match iface.addr {
                    IfAddr::V4(ref _addr) => true,
                    IfAddr::V6(_) => false,
                }
        })
        .map(|iface| match iface.addr {
            IfAddr::V4(ref addr) => addr.ip.to_string(),
            IfAddr::V6(_) => unreachable!(), // We've already filtered for V4 above
        })
        .unwrap_or_else(|| "127.0.0.1".to_string());

    println!(
        "Serving files from the current directory on http://{}:{}",
        ip, port
    );

    #[cfg(any(target_os = "windows"))] //, target_os = "macos"
    {
        // Open browser
        open::that(format!("http://{}:{}", ip, port)).unwrap();
    }

    let server_addr = ([0, 0, 0, 0], port);

    // Start the server
    warp::serve(routes).run(server_addr).await;
}
