use crate::message::Message;

#[derive(Debug, Default, Clone)]
pub struct Conversation(pub Vec<Message>);
