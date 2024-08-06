use tokio::{
    net::TcpListener,
    sync::{mpsc, Mutex}
};
use tokio_tungstenite::{
    accept_async,
    tungstenite::Message
};
use std::sync::Arc;
use futures::{
    stream::StreamExt,
    SinkExt
};


type Sender = mpsc::UnboundedSender<Message>;

struct ChannelManager {
    channels: Arc<Mutex<std::collections::HashMap<String, Vec<Sender>>>>,
}

impl ChannelManager {
    fn new() -> Self {
        ChannelManager {
            channels: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    async fn get_or_create_channel(&self, channel_name: String) {
        let mut channels = self.channels.lock().await;
        if !channels.contains_key(&channel_name) {
            channels.insert(channel_name.clone(), Vec::new());
        }
    }

    async fn add_sender_to_channel(&self, channel_name: String, sender: Sender) {
        let mut channels = self.channels.lock().await;
        if let Some(senders) = channels.get_mut(&channel_name) {
            senders.push(sender);
        }
    }

    async fn remove_sender_from_channel(&self, channel_name: String, sender: &Sender) {
        let mut channels = self.channels.lock().await;
        if let Some(senders) = channels.get_mut(&channel_name) {
            senders.retain(|s| s as *const _ != sender as *const _);
        }
    }

    async fn broadcast(&self, channel_name: String, message: Message) {
        let mut channels = self.channels.lock().await;
        if let Some(senders) = channels.get_mut(&channel_name) {
            for sender in senders.iter() {
                sender.send(message.clone()).expect("Failed to send message");
            }
        }
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    channel_manager: Arc<ChannelManager>,
) {
    let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");

    println!("New WebSocket Connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel();
    let channel_manager = channel_manager.clone();

    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            write.send(msg).await.expect("Failed to send message");
        }
    });

    let mut current_channel = String::new();
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if text.starts_with("CREATE_ROOM:") {
                    let room_name = &text[12..];
                    channel_manager.get_or_create_channel(room_name.to_string()).await;
                    channel_manager.add_sender_to_channel(room_name.to_string(), tx.clone()).await;
                    current_channel = room_name.to_string();
                } else if text.starts_with("JOIN_ROOM:") {
                    let room_name = &text[10..];
                    channel_manager.get_or_create_channel(room_name.to_string()).await;
                    channel_manager.add_sender_to_channel(room_name.to_string(), tx.clone()).await;
                    current_channel = room_name.to_string();
                } else if text.starts_with("LEAVE_ROOM:") {
                    let room_name = &text[11..];
                    channel_manager.remove_sender_from_channel(room_name.to_string(), &tx).await;
                    current_channel.clear();
                } else if text.starts_with("ROOM_MSG:") {
                    let parts: Vec<&str> = text[9..].splitn(3, ':').collect();
                    if parts.len() == 3 {
                        let room_name = parts[0];
                        let username = parts[1];
                        let message = parts[2];
                        channel_manager.broadcast(room_name.to_string(), Message::Text(format!("{} : {}", username, message))).await;
                    }
                }
            }
            Ok(_) => {
                // Handle other message
            }
            Err(e) => eprintln!("Error receiving message: {}", e),
        }
    }

    println!("WebSocket connection closed: {}", addr);

}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("webSocket server Started at ws://127.0.0.1:8080");
    
    let channel_manager = Arc::new(ChannelManager::new());

    while let Ok((stream, addr)) = listener.accept().await {
        let channel_manager: Arc<ChannelManager> = channel_manager.clone();
        tokio::spawn(handle_connection(stream, addr, channel_manager));
    }

    Ok(())
}