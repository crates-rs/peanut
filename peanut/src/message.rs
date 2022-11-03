#[derive(Debug)]
pub enum Message {
    Text(String),
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
