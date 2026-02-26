use serde::{Deserialize, Serialize};

/// The role of a participant in an agent dialogue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Agent,
    System,
}

/// A single message in an agent dialogue session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// Represents a multi-step agent dialogue session.
pub struct DialogueSession {
    messages: Vec<Message>,
}

impl DialogueSession {
    pub fn new() -> Self {
        DialogueSession { messages: Vec::new() }
    }

    /// Append a message from the user.
    pub fn add_user(&mut self, content: impl Into<String>) {
        self.messages.push(Message { role: Role::User, content: content.into() });
    }

    /// Append a message from the agent.
    pub fn add_agent(&mut self, content: impl Into<String>) {
        self.messages.push(Message { role: Role::Agent, content: content.into() });
    }

    /// Return all messages in the session.
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// Clear the session history.
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Serialize the session to JSON for transmission to an agent backend.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.messages)
    }

    /// Deserialize messages from a JSON string received from an agent backend.
    pub fn from_json(json: &str) -> Result<Vec<Message>, serde_json::Error> {
        serde_json::from_str(json)
    }
}

impl Default for DialogueSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_messages() {
        let mut session = DialogueSession::new();
        session.add_user("How do I list files?");
        session.add_agent("Use `ls -la` to list all files including hidden ones.");
        assert_eq!(session.messages().len(), 2);
        assert_eq!(session.messages()[0].role, Role::User);
        assert_eq!(session.messages()[1].role, Role::Agent);
    }

    #[test]
    fn test_clear_session() {
        let mut session = DialogueSession::new();
        session.add_user("hello");
        session.clear();
        assert!(session.messages().is_empty());
    }

    #[test]
    fn test_json_roundtrip() {
        let mut session = DialogueSession::new();
        session.add_user("test question");
        session.add_agent("test answer");
        let json = session.to_json().unwrap();
        let messages = DialogueSession::from_json(&json).unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].content, "test question");
        assert_eq!(messages[1].content, "test answer");
    }
}
