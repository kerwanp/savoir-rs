use std::collections::HashMap;

use crate::conversation::Conversation;

use super::ConversationStore;

#[derive(Debug, Default)]
pub struct InMemoryConversationStore {
    conversations: HashMap<String, Conversation>,
}

impl ConversationStore for InMemoryConversationStore {
    fn get_mut(&mut self, id: &str) -> Option<&mut Conversation> {
        self.conversations.get_mut(id)
    }

    fn create(&mut self, id: &str, conversation: Conversation) -> &mut Conversation {
        self.conversations.insert(id.to_string(), conversation);
        self.get_mut(id).unwrap()
    }
}
