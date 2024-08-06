# WebSocket Server and Client in Rust

This project provides a WebSocket server and client implementation in Rust. The server manages multiple chat rooms, allowing clients to create rooms, join rooms, send messages, and leave rooms. The client connects to the server and interacts with the chat rooms.

## Features

- **WebSocket Server**:
  - Manages chat rooms.
  - Handles client connections.
  - Broadcasts messages to clients in the same room.

- **WebSocket Client**:
  - Connects to the WebSocket server.
  - Allows users to create or join rooms.
  - Sends and receives messages in real-time.

## Prerequisites

- Rust (1.60 or later)
- Cargo (included with Rust)

## Getting Started

### Clone the Repository

First, clone the repository and navigate to the project directory:
    ```bash
    git clone https://github.com/SuryodayDevHub/websocket_rust.git
    cd websocket_rust
    ```

## Install Dependencies

Make sure you have the following dependencies in your `Cargo.toml`:
    ```bash
    [dependencies]
    tokio = { version = "1", features = ["full"] }
    tokio-tungstenite = "0.17"
    tungstenite = "0.15"
    futures = "0.3"
    futures-util = "0.3"
    ```

## Running the Server

To start the WebSocket server, use the following command:
    ```bash
    cargo run -- server
    ```

The server will listen for connections on ws://127.0.0.1:8080.

## Running the Client

To start the WebSocket client, use the following command:
    ```bash
    cargo run -- client
    ```

The client will connect to the WebSocket server at ws://127.0.0.1:8080.


## Running Both Server and Client

To run both the server and the client simultaneously, use the following command:
    ```bash
    cargo run -- both
    ```

This command will start both the server and the client in separate tasks.

## Usage

### Client Interaction

Once the client is running, you can interact with it using the following commands:

- **Create a Room**: Type `CREATE <room_name>` to create and join a new chat room.
- **Join a Room**: Type `JOIN <room_name>` to join an existing chat room.
- **Leave a Room**: Type `/leave` to leave the current chat room.
- **Send a Message**: Type your message and press Enter to send it to the current chat room.

### Example

1. Start the server in one terminal:

    ```bash
    cargo run -- server
    ```

2. In another terminal, start the client:

    ```bash
    cargo run -- client
    ```

3. Follow the prompts in the client to create or join a chat room and send messages.

## Code Structure

- `main.rs`: Entry point for running the server, client, or both.
- `server.rs`: Contains the WebSocket server logic.
- `client.rs`: Contains the WebSocket client logic.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to enhance the project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
