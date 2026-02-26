/// Represents a voice input transcript received from a voice recognition backend.
#[derive(Debug, Clone, PartialEq)]
pub struct VoiceTranscript {
    pub text: String,
    /// Confidence score in the range [0.0, 1.0].
    pub confidence: f32,
}

/// State of the voice input subsystem.
#[derive(Debug, Clone, PartialEq)]
pub enum VoiceState {
    Idle,
    Listening,
    Processing,
}

/// Manages voice input for hands-free interaction with the terminal or agent.
pub struct VoiceInput {
    pub state: VoiceState,
    /// Buffer of transcripts received in the current session.
    pub transcripts: Vec<VoiceTranscript>,
}

impl VoiceInput {
    pub fn new() -> Self {
        VoiceInput {
            state: VoiceState::Idle,
            transcripts: Vec::new(),
        }
    }

    /// Start listening for voice input.
    pub fn start_listening(&mut self) {
        self.state = VoiceState::Listening;
    }

    /// Stop listening and transition to Processing state.
    pub fn stop_listening(&mut self) {
        if self.state == VoiceState::Listening {
            self.state = VoiceState::Processing;
        }
    }

    /// Receive a transcript from the voice recognition backend and return
    /// to Idle state.
    pub fn receive_transcript(&mut self, text: impl Into<String>, confidence: f32) -> &VoiceTranscript {
        self.state = VoiceState::Idle;
        self.transcripts.push(VoiceTranscript {
            text: text.into(),
            confidence,
        });
        self.transcripts.last().unwrap()
    }

    /// Return the most recent transcript, if any.
    pub fn latest(&self) -> Option<&VoiceTranscript> {
        self.transcripts.last()
    }
}

impl Default for VoiceInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_state_transitions() {
        let mut vi = VoiceInput::new();
        assert_eq!(vi.state, VoiceState::Idle);
        vi.start_listening();
        assert_eq!(vi.state, VoiceState::Listening);
        vi.stop_listening();
        assert_eq!(vi.state, VoiceState::Processing);
        vi.receive_transcript("list all files", 0.95);
        assert_eq!(vi.state, VoiceState::Idle);
    }

    #[test]
    fn test_transcript_stored() {
        let mut vi = VoiceInput::new();
        vi.start_listening();
        vi.receive_transcript("run tests", 0.9);
        let t = vi.latest().unwrap();
        assert_eq!(t.text, "run tests");
        assert!((t.confidence - 0.9).abs() < f32::EPSILON);
    }
}
