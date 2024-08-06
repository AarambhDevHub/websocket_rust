use tokio::task;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <server|client|both>");
        return;
    }

    match args[1].as_str() {
        "server" => {
            if let Err(e) = server::start_server().await {
                eprintln!("Error starting server: {}", e);
            }
        }
        "client" => {
            if let Err(e) = client::start_client().await {
                eprintln!("Error starting client: {}", e);
            }
        }
        "both" => {
            let server_task = task::spawn(async {
                if let Err(e) = server::start_server().await {
                    eprintln!("Error starting server: {}", e);
                }
            });

            let client_task = task::spawn(async {
                if let Err(e) = client::start_client().await {
                    eprintln!("Error starting client: {}", e);
                }
            });

            let _ = tokio::join!(server_task, client_task);
        }
        _ => {
            eprintln!("Invalid argument. Use either'server', 'client', or 'both'.");
        }
    }
}

mod server;
mod client;