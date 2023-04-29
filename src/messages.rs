
pub enum Messages {
    Plus,
    Minus,
    Next,
    Prev,
    Copy,
    Quit,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "Plus" => Messages::Plus,
            "Minus" => Messages::Minus,
            "Next" => Messages::Next,
            "Prev" => Messages::Prev,
            "Copy" => Messages::Copy,
            "Quit" => Messages::Quit,
            _ => Messages::Unknown(event),
        }
    }
}
