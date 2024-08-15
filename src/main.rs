use tokio::net::TcpListener;
use warp::filters::log::Info;
use warp::Filter;

fn custom_log(info: Info) {
    let remote_addr = info
        .remote_addr()
        .map_or("unknown".to_string(), |addr| addr.to_string());
    eprintln!(
        "{} {} {} - {} - {}ms",
        info.method(),
        info.path(),
        info.status(),
        remote_addr,
        info.elapsed().as_millis()
    );
}

async fn find_available_port() -> std::io::Result<u16> {
    if let Ok(listener) = TcpListener::bind("0.0.0.0:80").await {
        return Ok(listener.local_addr()?.port());
    }

    // If port 80 is not available, bind to a random port
    let listener = TcpListener::bind("0.0.0.0:0").await?;
    Ok(listener.local_addr()?.port())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let www_root = "./public_html";

    let routes = warp::fs::dir(www_root)
        //.with(warp::filters::compression::brotli())
        .with(warp::log::custom(custom_log));

    let port = find_available_port().await?;

    println!(
        "Serving '{}' directory on http://localhost:{}\n\
         Put your HTML/CSS/JavaScript files in the '{}' folder.\n\
         Press Ctrl+C to quit...",
        www_root, port, www_root
    );

    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        if let Err(e) = open::that(format!("http://localhost:{}", port)) {
            eprintln!("Failed to open browser: {}", e);
        }
    }

    let server_addr = ([0, 0, 0, 0], port);

    // Start the server
    warp::serve(routes).run(server_addr).await;

    Ok(())
}
