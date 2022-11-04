use tokio::net::{TcpStream, ToSocketAddrs};
use super::{connection, message::{Message}};
pub async fn connect_server<T: ToSocketAddrs>(addr: T) -> anyhow::Result<()> {
    match TcpStream::connect(addr).await {
        Ok(socket) => {
          let (mut reader, mut writer) = connection::Connection::new(socket).split();
          writer.write_message(Message::Ping).await.unwrap();
          loop {
            match reader.read_message().await {
                Ok(message) => {
                    match message {
                        Message::Json(json) => {
                            log::debug!("recv text message: {}", json);
                        },
                        Message::Close(close_frame) => {
                            log::debug!("server close connected, err: {:?}", close_frame);
                            return Err(anyhow::anyhow!("server close connected"))
                        },
                        _ => ()
                    }
                },
                Err(err) => {
                    log::info!("server close connected, err: {:?}", err);
                    return Err(err)
                }
            }
          }
        },
        Err(e) => return Err(e.into()),
    }
    // Ok(())
}
