pub mod autocomplete;
pub mod highlight;
pub mod input;

pub use autocomplete::AutocompleteEngine;
pub use highlight::Highlighter;
pub use input::InputBuffer;

/// A completed command block stored in the session history.
#[derive(Debug, Clone)]
pub struct Block {
    /// The command text that was executed.
    pub command: String,
    /// The captured output of the command.
    pub output: String,
    /// Exit code returned by the command.
    pub exit_code: i32,
}

/// Manages the ordered list of command blocks for the current session.
pub struct BlockNavigator {
    blocks: Vec<Block>,
    /// Index of the currently selected block (for keyboard navigation).
    selected: Option<usize>,
}

impl BlockNavigator {
    pub fn new() -> Self {
        BlockNavigator {
            blocks: Vec::new(),
            selected: None,
        }
    }

    /// Append a new block and select it.
    pub fn push(&mut self, block: Block) {
        self.blocks.push(block);
        self.selected = Some(self.blocks.len() - 1);
    }

    /// Move selection to the previous block. Returns the newly selected block.
    pub fn prev(&mut self) -> Option<&Block> {
        match self.selected {
            Some(i) if i > 0 => {
                self.selected = Some(i - 1);
                self.blocks.get(i - 1)
            }
            _ => None,
        }
    }

    /// Move selection to the next block. Returns the newly selected block.
    pub fn next(&mut self) -> Option<&Block> {
        match self.selected {
            Some(i) if i + 1 < self.blocks.len() => {
                self.selected = Some(i + 1);
                self.blocks.get(i + 1)
            }
            _ => None,
        }
    }

    /// Return the currently selected block, if any.
    pub fn current(&self) -> Option<&Block> {
        self.selected.and_then(|i| self.blocks.get(i))
    }

    /// Return all blocks.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
}

impl Default for BlockNavigator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_block(cmd: &str) -> Block {
        Block { command: cmd.into(), output: String::new(), exit_code: 0 }
    }

    #[test]
    fn test_push_selects_latest() {
        let mut nav = BlockNavigator::new();
        nav.push(make_block("ls"));
        nav.push(make_block("pwd"));
        assert_eq!(nav.current().unwrap().command, "pwd");
    }

    #[test]
    fn test_prev_navigation() {
        let mut nav = BlockNavigator::new();
        nav.push(make_block("a"));
        nav.push(make_block("b"));
        let prev = nav.prev().unwrap();
        assert_eq!(prev.command, "a");
    }

    #[test]
    fn test_next_navigation() {
        let mut nav = BlockNavigator::new();
        nav.push(make_block("a"));
        nav.push(make_block("b"));
        nav.prev();
        let next = nav.next().unwrap();
        assert_eq!(next.command, "b");
    }

    #[test]
    fn test_prev_at_start_returns_none() {
        let mut nav = BlockNavigator::new();
        nav.push(make_block("a"));
        assert!(nav.prev().is_none());
    }
}
