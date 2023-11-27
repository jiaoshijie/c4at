use std::{collections::HashMap, net::SocketAddr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{tcp::OwnedWriteHalf, TcpListener, TcpStream},
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
};

type Result<T> = std::result::Result<T, ()>;

enum Message {
    Connected(OwnedWriteHalf, SocketAddr),
    Disconnected(SocketAddr),
    Message(Vec<u8>, SocketAddr),
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:9999";
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|err| eprintln!("ERROR: Can't listen to {}: {}", addr, err))?;
    println!("Listening on: {}", addr);

    let (sender, receiver) = unbounded_channel::<Message>();
    tokio::spawn(server(receiver));

    loop {
        let (mut socket, _) = listener
            .accept()
            .await
            .map_err(|err| eprintln!("ERROR: Accepting new client failed: {}", err))?;
        let ip = match socket.peer_addr() {
            Ok(ip) => ip,
            Err(err) => {
                eprintln!("ERROR: Get socket ip addr failed: {err}");
                let _ = socket
                    .shutdown()
                    .await
                    .map_err(|err| eprintln!("ERROR: shutdown client socket failed: {err}"));
                continue;
            }
        };
        let sender = sender.clone();
        tokio::spawn(client(socket, ip, sender));
    }
    // Ok(())
}

async fn server(mut receiver: UnboundedReceiver<Message>) -> Result<()> {
    let mut map: HashMap<SocketAddr, OwnedWriteHalf> = HashMap::new();
    while let Some(msg) = receiver.recv().await {
        match msg {
            Message::Connected(socket, addr) => {
                println!("INFO: `{addr}` connected");
                map.insert(addr, socket);
            }
            Message::Disconnected(addr) => {
                println!("INFO: `{addr}` disconnected");
                if let Some(mut socket) = map.remove(&addr) {
                    let _ = socket.shutdown().await.map_err(|err| {
                        eprintln!("ERROR: Shutdown socket in server side failed: {err}")
                    });
                }
            }
            Message::Message(msg, addr) => {
                println!("INFO: Got Message {msg:?} from {addr}");
                for (k_addr, socket) in map.iter_mut() {
                    if addr != *k_addr {
                        let _ = socket
                            .write_all(&msg)
                            .await
                            .map_err(|err| eprintln!("ERROR: write msg to all client, but failed in client {k_addr}: {err}"));
                    }
                }
            }
        }
    }
    Ok(())
}

async fn client(
    socket: TcpStream,
    addr: SocketAddr,
    sender: UnboundedSender<Message>,
) -> Result<()> {
    let (mut socket, writer) = socket.into_split();
    sender
        .send(Message::Connected(writer, addr))
        .map_err(|err| {
            // TODO: maybe writer.shutdown
            eprintln!("ERROR: Sending Connected({addr}) message to server failed: {err}");
        })?;
    let mut buffer = [0; 64];
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                let _ = sender.send(Message::Disconnected(addr)).map_err(|err| {
                    eprintln!("ERROR: Sending Disconnected message to server failed: {err}")
                });
                return Ok(());
            }
            Ok(n) => {
                // TODO: For now i just ignore this error
                let _ = sender
                    .send(Message::Message(buffer[0..n].to_vec(), addr))
                    .map_err(|err| {
                        eprintln!("ERROR: Sending Message to server failed: {err}");
                    });
            }
            Err(err) => {
                eprintln!("ERROR: Client read message failed: {err}");
                let _ = sender.send(Message::Disconnected(addr)).map_err(|err| {
                    eprintln!("ERROR: Sending Disconnected message to server failed: {err}")
                });
                return Err(());
            }
        }
    }
}
