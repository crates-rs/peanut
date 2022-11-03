use super::{connection, message};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub async fn start_server<T: ToSocketAddrs>(addr: T) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (socket, addr) = listener.accept().await?;
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
            Ok(message) => match message {
                message::Message::Text(message) => {
                    log::debug!("recv text message: {}", message);
                    writer.write_text(&message).await?;
                    log::debug!("send back message: {}", message);
                }
                message::Message::Close(close_frame) => {
                    log::debug!("client disconnected, err: {:?}", close_frame);
                    return Err(anyhow::anyhow!("client disconnected"));
                }
                _ => (),
            },
            Err(err) => {
                log::debug!("client disconnected, err: {:?}", err);
                return Err(err);
            }
        }
    }
}
