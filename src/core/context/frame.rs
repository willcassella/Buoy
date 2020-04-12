use crate::core::message::{Inbox, Message, MessageMap};

pub struct FrameContext {
    incoming_messages: MessageMap,
}

impl FrameContext {
    pub fn new(incoming_messages: MessageMap) -> Self {
        FrameContext { incoming_messages }
    }

    pub fn read_message<T: Message>(&self, inbox: Inbox<T>) -> Option<T> {
        self.incoming_messages.read(inbox)
    }
}
