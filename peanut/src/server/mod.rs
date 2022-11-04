use super::{connection, message::{Message}};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub async fn start_server<T: ToSocketAddrs>(addr: T) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    loop {
        println!("recv msg test1");
        let (socket, addr) = listener.accept().await?;
        println!("recv msg test2");
        tokio::spawn(async move {
            if process(socket, addr).await.is_err() {
                log::info!("disconnect: {:?}", addr);
            }
        });
    }
}

async fn process(socket: TcpStream, _addr: SocketAddr) -> anyhow::Result<()> {
    let (mut reader, mut writer) = connection::Connection::new(socket).split();

    loop {
        match reader.read_message().await {
            Ok(message) => {
                match message {
                    Message::Json(json) => {
                        log::debug!("recv text message: {}", json);
                        log::debug!("send back message: {}", json);
                        writer.write_message(Message::Json(json)).await?;
                    },
                    Message::Close(close_frame) => {
                        log::debug!("client disconnected, err: {:?}", close_frame);
                        return Err(anyhow::anyhow!("client disconnected"));
                    },
                    _ => {},
                }
            },
            Err(err) => {
                log::debug!("client disconnected, err: {:?}", err);
                return Err(err);
            }
        }
    }
}
