use std::str;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};


#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6667").await.expect("TcpListener couldn't bind to port 6667! Is this port already bound?");
    
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buf = vec![0; 512];

    loop {
        match socket.read(&mut buf).await {
            // TODO: Handle disconnects
            // Client disconnected
            Ok(0) => return,
            Ok(_n) => {
                let inc = str::from_utf8(&buf).expect("valid utf-8").replace("\0", "");
                let unparsed_packets: Vec<&str> = inc
                    .split("\r\n")
                    .filter(|s| s != &"")
                    .collect();
                for unparsed_packet in unparsed_packets {
                    // TODO: Parse and handle packet
                    println!("{:?}", unparsed_packet);
                }
            },
            // Unexpected socket error. Treat client as disconnected
            Err(_) => return
        }
    }
}
