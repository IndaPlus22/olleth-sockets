use mpsc::TryRecvError;
use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc,
    thread,
    time::Duration,
};

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TcpStream::connect(LOCAL)?;
    client.set_nonblocking(true)?;

    let (tx, rx) = mpsc::channel::<String>();

    println!("Enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    thread::spawn(move || loop {
        let mut size_buf = [0; 4];
        if let Ok(_) = client.read_exact(&mut size_buf) {
            let msg_size = u32::from_be_bytes(size_buf) as usize;
            let mut msg_buf = vec![0; msg_size];
            if let Ok(_) = client.read_exact(&mut msg_buf) {
                // Ignore received messages
            }
        }

        match rx.try_recv() {
            Ok(mut msg) => {
                let mut color = None;
                if msg.starts_with('/') {
                    let parts: Vec<&str> = msg.splitn(2, ' ').collect();
                    if let Some(tag) = parts[0].strip_prefix('/') {
                        let styled_msg = match tag {
                            "red" => format!("\x1B[31m{}\x1B[0m", parts[1]),
                            "green" => format!("\x1B[32m{}\x1B[0m", parts[1]),
                            "blue" => format!("\x1B[34m{}\x1B[0m", parts[1]),
                            "yellow" => format!("\x1B[33m{}\x1B[0m", parts[1]),
                            "magenta" => format!("\x1B[35m{}\x1B[0m", parts[1]),
                            "cyan" => format!("\x1B[36m{}\x1B[0m", parts[1]),
                            "purple" => format!("\x1B[35m{}\x1B[0m", parts[1]),
                            "bright_red" => format!("\x1B[91m{}\x1B[0m", parts[1]),
                            "bright_green" => format!("\x1B[92m{}\x1B[0m", parts[1]),
                            "bright_blue" => format!("\x1B[94m{}\x1B[0m", parts[1]),
                            "bright_yellow" => format!("\x1B[93m{}\x1B[0m", parts[1]),
                            "bright_magenta" => format!("\x1B[95m{}\x1B[0m", parts[1]),
                            "bright_cyan" => format!("\x1B[96m{}\x1B[0m", parts[1]),
                            "bright_purple" => format!("\x1B[95m{}\x1B[0m", parts[1]),
                            "bold" => format!("\x1B[1m{}\x1B[0m", parts[1]),
                            "underline" => format!("\x1B[4m{}\x1B[0m", parts[1]),
                            "italic" => format!("\x1B[3m{}\x1B[0m", parts[1]),
                            _ => msg,
                        };
                        msg = format!("{}: {}", username, styled_msg);
                        color = Some(styled_msg);
                    }
                }
                let msg_bytes = msg.as_bytes();
                let msg_size = (msg_bytes.len() as u32).to_be_bytes();
                client.write_all(&msg_size).expect("msg");
                client.write_all(msg_bytes).expect("msg");

                if let Some(colored_msg) = color {
                    println!("{}", colored_msg);
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a message (use color tags like '/red' at the end of the message to change the color):");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff)?;
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }

    println!("Bye bye");

    Ok(())
}