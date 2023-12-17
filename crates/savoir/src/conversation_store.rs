use crate::conversation::Conversation;
use std::fmt::Debug;

pub mod in_memory;

pub trait ConversationStore: Send + Sync + Debug {
    fn get_mut(&mut self, id: &str) -> Option<&mut Conversation>;
    fn create(&mut self, id: &str, conversation: Conversation) -> &mut Conversation;
}
