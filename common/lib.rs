use chrono::NaiveDateTime
use serde::{Serialize,Deserialize}

pub enum WebSocketMessageType{
    NewMessage,
    UsersList,
}

pub struct WebSocketMessage{
    pub message_type:WebSocketMessageType,
    pub message:Option<ChatMessage>,
    pub users:Option<Vec<String>>,
}


#[derive(Serialize,Deserialize)]
pub struct ChatMessage{
    pub message:String,
    pub author:String,
    pub created_at:NaiveDateTime
}