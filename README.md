# Chat Client and Server

This is a simple chat client and server implementation in Rust. It allows multiple clients to connect to the server and exchange messages.

## Features

- Client and server communication over TCP/IP.
- Non-blocking I/O for concurrent message sending and receiving.
- Color tagging to style messages (e.g., /red, /green, /blue, etc.).
- Username assignment for clients.

## Usage

### Server

1. Open a terminal and navigate to the server directory.
2. Build the server using the following command:

    ```shell
    cargo build
    ```

3. Start the server by running:

    ```shell
    cargo run
    ```

4. The server will start listening for incoming connections on `127.0.0.1:6000`.

### Client

1. Open a terminal and navigate to the client directory.
2. Build the client using the following command:

    ```shell
    cargo build
    ```

3. Start the client by running:

    ```shell
    cargo run
    ```

4. Enter your desired username when prompted.
5. You can now start sending and receiving messages in the chat.

### Sending Messages

- To send a regular message, simply type your message and press Enter.
- To send a styled message, prefix your message with a style tag. For example, to send a red message, type "/red Your message here" and press Enter. 

Available Colors:

- Red
- Green
- Blue
- Yellow
- Magenta
- Cyan
- Purple
- Bright Red
- Bright Green
- Bright Blue
- Bright Yellow
- Bright Magenta
- Bright Cyan
- Bright Purple

Formatting Options:

- bold
- Italic
- Underline
### Quitting the Client

- To quit the client, type `:quit` and press Enter. This will disconnect you from the server.