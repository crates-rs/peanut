#[derive(Debug)]
pub enum Message {
    Ping,
    Pong,
    Json(serde_json::Value),
    Close(Option<CloseFrame>),
}

#[derive(Debug)]
pub struct CloseFrame {
    pub code: CloseCode,
    pub reason: String,
}

#[derive(Debug)]
pub enum CloseCode {
    Normal,
}
