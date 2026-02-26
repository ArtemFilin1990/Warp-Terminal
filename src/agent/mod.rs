pub mod dialogue;

pub use dialogue::{DialogueSession, Message, Role};

/// The operational mode of the terminal.
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalMode {
    /// Pure command-line mode: user types shell commands.
    Command,
    /// Agent dialogue mode: multi-step conversation with an AI agent.
    Dialogue,
}

/// Manages mode switching and the active agent session.
pub struct AgentMode {
    /// Current active mode.
    pub mode: TerminalMode,
    /// The agent dialogue session (active in `Dialogue` mode).
    pub session: DialogueSession,
    /// Identifier of the connected agent backend (e.g. "claude", "codex").
    pub agent_id: Option<String>,
}

impl AgentMode {
    pub fn new() -> Self {
        AgentMode {
            mode: TerminalMode::Command,
            session: DialogueSession::new(),
            agent_id: None,
        }
    }

    /// Switch to agent dialogue mode and optionally set the agent backend.
    pub fn enter_dialogue(&mut self, agent_id: impl Into<String>) {
        self.mode = TerminalMode::Dialogue;
        self.agent_id = Some(agent_id.into());
    }

    /// Switch back to command mode without clearing the session history.
    pub fn enter_command(&mut self) {
        self.mode = TerminalMode::Command;
    }

    /// Toggle between `Command` and `Dialogue` modes.
    pub fn toggle(&mut self) {
        match self.mode {
            TerminalMode::Command => {
                self.mode = TerminalMode::Dialogue;
            }
            TerminalMode::Dialogue => {
                self.mode = TerminalMode::Command;
            }
        }
    }

    /// Return `true` when the terminal is in dialogue mode.
    pub fn is_dialogue(&self) -> bool {
        self.mode == TerminalMode::Dialogue
    }
}

impl Default for AgentMode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mode_is_command() {
        let agent = AgentMode::new();
        assert_eq!(agent.mode, TerminalMode::Command);
    }

    #[test]
    fn test_enter_dialogue() {
        let mut agent = AgentMode::new();
        agent.enter_dialogue("claude");
        assert_eq!(agent.mode, TerminalMode::Dialogue);
        assert_eq!(agent.agent_id.as_deref(), Some("claude"));
    }

    #[test]
    fn test_toggle_mode() {
        let mut agent = AgentMode::new();
        agent.toggle();
        assert_eq!(agent.mode, TerminalMode::Dialogue);
        agent.toggle();
        assert_eq!(agent.mode, TerminalMode::Command);
    }

    #[test]
    fn test_enter_command_from_dialogue() {
        let mut agent = AgentMode::new();
        agent.enter_dialogue("codex");
        agent.enter_command();
        assert!(!agent.is_dialogue());
    }

    #[test]
    fn test_session_persists_across_mode_switch() {
        let mut agent = AgentMode::new();
        agent.enter_dialogue("claude");
        agent.session.add_user("hello");
        agent.enter_command();
        // Re-enter dialogue - session should still have the message
        agent.enter_dialogue("claude");
        assert_eq!(agent.session.messages().len(), 1);
    }
}
