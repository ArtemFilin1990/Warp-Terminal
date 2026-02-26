pub mod selection;
pub mod voice;

pub use selection::{SelectionRegistry, SelectionResource};
pub use voice::VoiceInput;

/// Identifies a supported agent backend.
#[derive(Debug, Clone, PartialEq)]
pub enum AgentBackend {
    /// Anthropic's Claude Code agent.
    ClaudeCode,
    /// OpenAI Codex agent.
    Codex,
    /// The Oz agent (Warp-native).
    Oz,
    /// A custom agent identified by name.
    Custom(String),
}

impl AgentBackend {
    /// Parse a backend identifier string into an `AgentBackend` variant.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "claude" | "claude-code" => AgentBackend::ClaudeCode,
            "codex" => AgentBackend::Codex,
            "oz" => AgentBackend::Oz,
            other => AgentBackend::Custom(other.to_owned()),
        }
    }

    /// Return the canonical identifier string for this backend.
    pub fn id(&self) -> &str {
        match self {
            AgentBackend::ClaudeCode => "claude-code",
            AgentBackend::Codex => "codex",
            AgentBackend::Oz => "oz",
            AgentBackend::Custom(s) => s.as_str(),
        }
    }
}

/// Aggregates all agent integration features available in the terminal.
pub struct AgentIntegration {
    pub voice: VoiceInput,
    pub selections: SelectionRegistry,
    pub backend: Option<AgentBackend>,
}

impl AgentIntegration {
    pub fn new() -> Self {
        AgentIntegration {
            voice: VoiceInput::new(),
            selections: SelectionRegistry::new(),
            backend: None,
        }
    }

    /// Connect to a specific agent backend.
    pub fn connect(&mut self, backend: AgentBackend) {
        self.backend = Some(backend);
    }

    /// Build a prompt from a raw text string by expanding any @-references.
    pub fn build_prompt(&self, raw: &str) -> String {
        self.selections.expand_prompt(raw)
    }
}

impl Default for AgentIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_from_str() {
        assert_eq!(AgentBackend::from_str("claude"), AgentBackend::ClaudeCode);
        assert_eq!(AgentBackend::from_str("codex"), AgentBackend::Codex);
        assert_eq!(AgentBackend::from_str("oz"), AgentBackend::Oz);
        assert_eq!(
            AgentBackend::from_str("my-agent"),
            AgentBackend::Custom("my-agent".into())
        );
    }

    #[test]
    fn test_backend_id() {
        assert_eq!(AgentBackend::ClaudeCode.id(), "claude-code");
        assert_eq!(AgentBackend::Codex.id(), "codex");
    }

    #[test]
    fn test_connect_backend() {
        let mut integration = AgentIntegration::new();
        integration.connect(AgentBackend::ClaudeCode);
        assert_eq!(integration.backend, Some(AgentBackend::ClaudeCode));
    }
}
