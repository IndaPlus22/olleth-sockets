use std::{
    io::{ErrorKind, Read, Write},
    net::TcpListener,
    sync::mpsc,
    thread,
};

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(std::time::Duration::from_millis(100));
}

fn handle_client(
    mut socket: std::net::TcpStream,
    addr: std::net::SocketAddr,
    tx: mpsc::Sender<String>,
) {
    thread::spawn(move || {
        loop {
            let mut size_buf = [0; 4];
            if let Ok(_) = socket.read_exact(&mut size_buf) {
                let msg_size = u32::from_be_bytes(size_buf) as usize;
                let mut msg_buf = vec![0; msg_size];
                if let Ok(_) = socket.read_exact(&mut msg_buf) {
                    let msg = String::from_utf8_lossy(&msg_buf);
                    println!("{}", msg);
                    tx.send(msg.into_owned()).expect("Failed to send message to rx");
                }
            }

            sleep();
        }
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind(LOCAL)?;
    server.set_nonblocking(true)?;

    let (tx, rx) = mpsc::channel::<String>();
    let mut clients = vec![];

    loop {
        if let Ok((socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            clients.push(socket.try_clone()?);
            handle_client(socket, addr, tx.clone());
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let msg_bytes = msg.as_bytes();
                    let msg_size = (msg_bytes.len() as u32).to_be_bytes();

                    if let Err(_) = client.write_all(&msg_size) {
                        return None;
                    }

                    if let Err(_) = client.write_all(msg_bytes) {
                        return None;
                    }

                    Some(client)
                })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}
