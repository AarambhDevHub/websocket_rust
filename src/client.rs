use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
};
use futures::{
    stream::StreamExt,
    SinkExt,
};
use std::{
    error::Error, io::{self, Write}, sync::Arc
};

pub async fn start_client() -> Result<(), Box<dyn Error>> {
    let (ws_stream, _) = connect_async("ws://127.0.0.1:8080").await?;
    println!("Connection to WebSocket server");

    let (mut write, mut read) = ws_stream.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    let message = Arc::new(Mutex::new(Vec::new()));

    tokio::spawn({
        let message = message.clone();
        async move {
            while let Some(msg) = rx.recv().await {
                write.send(msg).await.expect("Failed to send message");
            }
        }
    });

    tokio::spawn({
        let message = message.clone();
        async move{
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let mut msgs = message.lock().await;

                        if !text.starts_with("ROOM_MSG:") {
                            msgs.push(text.clone());
                            println!("{}", text);
                        }
                    }
                    Ok(Message::Binary(_)) => {
                        // Handle Binary message if needed
                    }
                    Err(e) => eprintln!("Error receiving message: {}", e),
                    _ => {
                        println!("Received an unknown message type");
                    }
                }
            }
        }
    });

    print!("Enter your username: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    println!("Do you want to create a room or join a room?");
    println!("Type 'CREATE <room_name>' to create a room");
    println!("Type 'JOIN <room_name>' to join a room");

    let mut  current_room = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.starts_with("CREATE ") {
            current_room = input[7..].to_string();
            tx.send(Message::Text(format!("CREATE_ROOM:{}", current_room)))
                .expect("Failed to send message");
            println!("You created and joined room: {}", current_room);
            break;
        } else if input.starts_with("JOIN ") {
            current_room = input[5..].to_string();
            tx.send(Message::Text(format!("JOIN_ROOM:{}", current_room)))
                .expect("Failed to send message");
            println!("You joined room: {}", current_room);
            
            let msgs = message.lock().await;
            println!("--- Pervious message ---");
            for msg in msgs.iter() {
                println!("{}", msg);
            }
            println!("------------------------");
            break;            
        } else {
            println!("Invalid command. please type 'CREATE <room_name>' or 'JOIN <room_name>'.")
        }
    }

    println!("You can now chat in room: {}", current_room);

    loop {
        print!("{} > ", username);
        io::stdout().flush()?;
        let mut message = String::new();
        io::stdin().read_line(&mut message)?;
        let message = message.trim();

        if message == "/leave" {
            tx.send(Message::Text(format!("LEAVE_ROOM: {}", current_room)))
            .expect("Failed to send message");
            println!("You left room: {}", current_room);
            break;
        }

        tx.send(Message::Text(format!("ROOM_MSG:{}:{}:{}", current_room, username, message)))
            .expect("Failed to send message")
    }

    Ok(())
}