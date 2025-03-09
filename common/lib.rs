use chrono::NaiveDateTime
use serde::{Serialize,Deserialize}

#[derive(Serialize,Deserialize)]
pub struct ChatMessage{
    pub message:String,
    pub author:String,
    pub created_at:NaiveDateTime
}